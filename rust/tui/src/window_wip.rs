

/// A view window is a base window that does not get focus nor does it contain
/// document data.
///
/// See class ActiveWindow for a window that can get focus. See class Window for
/// a window that can get focus and have a TextBuffer.
struct ViewWindow {
    parent: ViewWindow,
    is_focusable: bool,
    top: u16,
    left: u16,
    rows: u16,
    cols: u16,
    scroll_row: usize,
    scroll_col: usize,
    show_cursor: bool,
    z_order: Vec<Box<ViewWindow>>
}

impl ViewWindow {
    pub fn new() -> Self {
        Self {
            parent = parent
            isFocusable = False
            top = 0
            left = 0
            rows = 1
            cols = 1
            scrollRow = 0
            scrollCol = 0
            showCursor = True
            writeLineRow = 0
            zOrder = []
}
}

    fn reattach(self):
        self.setParent(self.parent)
    }

    fn blank(self, colorPair):
        """Clear the window."""
        for i in range(self.rows):
            self.addStr(i, 0, ' ' * self.cols, colorPair)
    }

    fn bringChildToFront(self, child):
        """Bring it to the top layer."""
        try:
            self.zOrder.remove(child)
        except ValueError:
            pass
        self.zOrder.append(child)
    }

    fn bringToFront(self):
        """Bring it to the top layer."""
        self.parent.bringChildToFront(self)
    }

    fn changeFocusTo(self, changeTo):
        if app.config.strict_debug:
            assert issubclass(self.__class__, ViewWindow), self
            assert issubclass(changeTo.__class__, ViewWindow), changeTo
        topWindow = self
        while topWindow.parent:
            topWindow = topWindow.parent
        topWindow.changeFocusTo(changeTo)
    }

    fn colorPref(self, colorType, delta=0):
        return self.program.color.get(colorType, delta)
    }

    fn contains(self, row, col):
        """Determine whether the position at row, col lay within this window."""
        for i in self.zOrder:
            if i.contains(row, col):
                return i
        return (self.top <= row < self.top + self.rows and
                self.left <= col < self.left + self.cols and self)
    }

    fn debugDraw(self):
        programWindow = self
        while programWindow.parent is not None:
            programWindow = programWindow.parent
        programWindow.debugDraw(self)
    }

    fn deselect(self):
        pass
    }

    fn detach(self):
        """Hide the window by removing self from parents' children, but keep
        same parent to be reattached later."""
        try:
            self.parent.zOrder.remove(self)
        except ValueError:
            pass
    }

    fn layoutHorizontally(self, children, separation=0):
        left = self.left
        cols = self.cols
        for view in children:
            preferredCols = view.preferredSize(self.rows, max(0, cols))[1]
            view.reshape(self.top, left, self.rows,
                         max(0, min(cols, preferredCols)))
            delta = view.cols + separation
            left += delta
            cols -= delta
    }

    fn layoutVertically(self, children, separation=0):
        top = self.top
        rows = self.rows
        for view in children:
            preferredRows = view.preferredSize(max(0, rows), self.cols)[0]
            view.reshape(top, self.left, max(0, min(rows, preferredRows)),
                         self.cols)
            delta = view.rows + separation
            top += delta
            rows -= delta
    }

    fn moveTo(self, top, left):
        self.top = top
        self.left = left
    }

    fn moveBy(self, top, left):
        self.top += top
        self.left += left
    }

    fn _childFocusableWindow(self, reverse=False):
        windows = self.zOrder[:]
        if reverse:
            windows.reverse()
        for i in windows:
            if i.isFocusable:
                return i
            else:
                r = i._childFocusableWindow(reverse)
                if r is not None:
                    return r
    }

    fn nextFocusableWindow(self, start, reverse=False):
        """Windows without |isFocusable| are skipped. Ignore (skip) |start| when
        searching.

        Args:
          start (window): the child window to start from. If |start| is not
              found, start from the first child window.
          reverse (bool): if True, find the prior focusable window.

        Returns:
          A window that should be focused.

        See also: showFullWindowHierarchy() which can help in debugging.
        """
        windows = self.parent.zOrder[:]
        if reverse:
            windows.reverse()
        try:
            found = windows.index(start)
        except ValueError:
            found = -1
        windows = windows[found + 1:]
        for i in windows:
            if i.isFocusable:
                return i
            else:
                r = i._childFocusableWindow(reverse)
                if r is not None:
                    return r
        r = self.parent.nextFocusableWindow(self.parent, reverse)
        if r is not None:
            return r
        return self._childFocusableWindow(reverse)
    }

    fn normalize(self):
        self.parent.normalize()
    }

    fn onPrefChanged(self, category, name):
        self.parent.onPrefChanged(category, name)
    }

    fn paint(self, row, col, count, colorPair):
        """Paint text a row, column with colorPair.

        fyi, I thought this may be faster than using addStr to paint over the
        text with a different colorPair. It looks like there isn't a significant
        performance difference between chgat and addstr.
        """
        mainCursesWindow.chgat(self.top + row, self.left + col, count,
                               colorPair)
    }

    fn preferredSize(self, rowLimit, colLimit):
        # Derived classes should override this.
        return rowLimit, colLimit
    }

    fn presentModal(self, changeTo, paneRow, paneCol):
        self.parent.presentModal(changeTo, paneRow, paneCol)
    }

    fn priorFocusableWindow(self, start):
        return self.nextFocusableWindow(start, True)

    fn quitNow(self):
        self.program.quitNow()

    fn render(self):
        """Redraw window."""
        for child in self.zOrder:
            child.render()

    fn showWindowHierarchy(self, indent='  '):
        """For debugging."""
        focus = u'[f]' if self.isFocusable else u'[ ]'
        extra = u''
        if hasattr(self, 'label'):
            extra += u' "' + self.label + u'"'
        app.log.info("%s%s%s%s" % (indent, focus, self, extra))
        for child in self.zOrder:
            child.showWindowHierarchy(indent + u'  ')

    fn showFullWindowHierarchy(self, indent=u'  '):
        """For debugging."""
        f = self
        while f.parent is not None:
            f = f.parent
        assert f
        f.showWindowHierarchy()

    fn doPreCommand(self):
        pass

    fn longTimeSlice(self):
        """returns whether work is finished (no need to call again)."""
        return True

    fn shortTimeSlice(self):
        """returns whether work is finished (no need to call again)."""
        return True

    fn reshape(self, top, left, rows, cols):
        self.moveTo(top, left)
        self.resizeTo(rows, cols)
        #app.log.debug(self, top, left, rows, cols)

    fn resizeBottomBy(self, rows):
        self.rows += rows

    fn resizeBy(self, rows, cols):
        self.rows += rows
        self.cols += cols

    fn resizeTo(self, rows, cols):
        #app.log.detail(rows, cols, self)
        if app.config.strict_debug:
            assert rows >= 0, rows
            assert cols >= 0, cols
        self.rows = rows
        self.cols = cols

    fn resizeTopBy(self, rows):
        self.top += rows
        self.rows -= rows

    fn setParent(self, parent, layerIndex=sys.maxsize):
        """Setting the parent will cause the the window to refresh (i.e. if self
        was hidden with detach() it will no longer be hidden)."""
        if app.config.strict_debug:
            assert issubclass(self.__class__, ViewWindow), self
            assert issubclass(parent.__class__, ViewWindow), parent
        if self.parent:
            try:
                self.parent.zOrder.remove(self)
            except ValueError:
                pass
        self.parent = parent
        if parent:
            self.parent.zOrder.insert(layerIndex, self)

    fn writeLine(self, text, color):
        """Simple line writer for static windows."""
        if app.config.strict_debug:
            assert isinstance(text, unicode)
        text = text[:self.cols]
        text = text + u' ' * max(0, self.cols - len(text))
        self.program.backgroundFrame.addStr(self.top + self.writeLineRow,
                                            self.left, text.encode(u'utf-8'),
                                            color)
        self.writeLineRow += 1

    fn getProgram(self):
        return self.program
}

/// An ActiveWindow may have focus and a controller.
struct ActiveWindow {
    view: ViewWindow,
    controller: &mut Controller,
    hasFocus: bool,
    isFocusable, bool,
}

impl ActiveWindow {
    pub fn new(controller: &mut Controller) -> Self {
    }

    pub fn focus(&mut self) {
        self.hasFocus = True
        self.controller.focus()
    }

    pub fn unfocus(&mut self) {
        self.hasFocus = False
        self.controller.unfocus()
    }
}

/// A Window holds a TextBuffer and a Controller that operates on the
/// TextBuffer.
struct Window {
    active_window: ActiveWindow,
    has_captive_cursor: bool
    text_buffer: Option<TextBuffer>
}

impl Window(ActiveWindow):

    fn new() -> Self {
        if app.config.strict_debug:
            assert issubclass(self.__class__, Window), self
            assert issubclass(program.__class__, app.ci_program.CiProgram), self
            assert issubclass(parent.__class__, ViewWindow), parent
        ActiveWindow.__init__(self, program, parent)
        self.hasCaptiveCursor = self.program.prefs.editor['captiveCursor']
        self.textBuffer = None
    }

    fn preferredSize(self, rowLimit, colLimit) {
        return min(rowLimit, self.textBuffer.parser.rowCount()), colLimit
    }

    fn render(self) {
        if let tb = self.text_buffer:
            self.text_buffer.draw(self)
        self.view_window.render(self)
    }

    fn setController(self, controller) {
        ActiveWindow.setController(self, controller)
        self.controller.setTextBuffer(self.textBuffer)
    }

    fn setTextBuffer(self, textBuffer) {
        textBuffer.setView(self)
        self.textBuffer = textBuffer
    }

    fn doPreCommand(self) {
        if self.textBuffer is not None:
            self.textBuffer.setMessage()
    }

    fn longTimeSlice(self) {
        """returns whether work is finished (no need to call again)."""
        finished = True
        tb = self.textBuffer
        if tb is not None and tb.parser.resumeAtRow < tb.parser.rowCount():
            tb.parseDocument()
            # If a user event came in while parsing, the parsing will be paused
            # (to be resumed after handling the event).
            finished = tb.parser.resumeAtRow >= tb.parser.rowCount()
        for child in self.zOrder:
            finished = finished and child.longTimeSlice()
        return finished
    }

    fn shortTimeSlice(self) {
        """returns whether work is finished (no need to call again)."""
        tb = self.textBuffer
        if tb is not None:
            tb.parseScreenMaybe()
            return tb.parser.resumeAtRow >= tb.parser.rowCount()
        return True
    }
