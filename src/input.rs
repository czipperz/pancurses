#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Input {
    Character(char),
    KeyCodeYes,

    KeyBreak,
    KeyDown,
    KeyUp,
    KeyLeft,
    KeyRight,
    KeyHome,
    KeyBackspace,

    KeyF0,
    KeyF1,
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
    KeyF13,
    KeyF14,
    KeyF15,

    KeyDL,
    KeyIL,
    KeyDC,
    KeyIC,
    KeyEIC,
    KeyClear,
    KeyEOS,
    KeyEOL,
    KeySF,
    KeySR,
    KeyNPage,
    KeyPPage,
    KeySTab,
    KeyCTab,
    KeyCATab,
    KeyEnter,
    KeySReset,
    KeyReset,
    KeyPrint,
    KeyLL,
    KeyAbort,
    KeySHelp,
    KeyLHelp,
    KeyBTab,
    KeyBeg,
    KeyCancel,
    KeyClose,
    KeyCommand,
    KeyCopy,
    KeyCreate,
    KeyEnd,
    KeyExit,
    KeyFind,
    KeyHelp,
    KeyMark,
    KeyMessage,
    KeyMove,
    KeyNext,
    KeyOpen,
    KeyOptions,
    KeyPrevious,
    KeyRedo,
    KeyReference,
    KeyRefresh,
    KeyReplace,
    KeyRestart,
    KeyResume,
    KeySave,
    KeySBeg,
    KeySCancel,
    KeySCommand,
    KeySCopy,
    KeySCreate,
    KeySDC,
    KeySDL,
    KeySelect,
    KeySEnd,
    KeySEOL,
    KeySExit,
    KeySFind,
    KeySHome,
    KeySIC,

    KeySLeft,
    KeySMessage,
    KeySMove,
    KeySNext,
    KeySOptions,
    KeySPrevious,
    KeySPrint,
    KeySRedo,
    KeySReplace,
    KeySRight,
    KeySResume,
    KeySSave,
    KeySSuspend,
    KeySUndo,
    KeySuspend,
    KeyUndo,

    KeyResize,
    KeyEvent,
    KeyMouse,

    KeyA1,
    KeyA3,
    KeyB2,
    KeyC1,
    KeyC3,
}