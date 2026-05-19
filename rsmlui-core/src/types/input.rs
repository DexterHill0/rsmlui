use bitflags::bitflags;
use rsmlui_macros::sys_cast;
use rsmlui_sys::{Rml_Input_KeyIdentifier, Rml_Input_KeyModifier};

#[non_exhaustive]
#[sys_cast(enum(from = Rml_Input_KeyIdentifier, repr = u8))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyCode {
    #[sys(KI_UNKNOWN)]
    Unknown = 0,
    #[sys(KI_SPACE)]
    Space = 1,
    #[sys(KI_0)]
    Num0 = 2,
    #[sys(KI_1)]
    Num1 = 3,
    #[sys(KI_2)]
    Num2 = 4,
    #[sys(KI_3)]
    Num3 = 5,
    #[sys(KI_4)]
    Num4 = 6,
    #[sys(KI_5)]
    Num5 = 7,
    #[sys(KI_6)]
    Num6 = 8,
    #[sys(KI_7)]
    Num7 = 9,
    #[sys(KI_8)]
    Num8 = 10,
    #[sys(KI_9)]
    Num9 = 11,
    #[sys(KI_A)]
    A = 12,
    #[sys(KI_B)]
    B = 13,
    #[sys(KI_C)]
    C = 14,
    #[sys(KI_D)]
    D = 15,
    #[sys(KI_E)]
    E = 16,
    #[sys(KI_F)]
    F = 17,
    #[sys(KI_G)]
    G = 18,
    #[sys(KI_H)]
    H = 19,
    #[sys(KI_I)]
    I = 20,
    #[sys(KI_J)]
    J = 21,
    #[sys(KI_K)]
    K = 22,
    #[sys(KI_L)]
    L = 23,
    #[sys(KI_M)]
    M = 24,
    #[sys(KI_N)]
    N = 25,
    #[sys(KI_O)]
    O = 26,
    #[sys(KI_P)]
    P = 27,
    #[sys(KI_Q)]
    Q = 28,
    #[sys(KI_R)]
    R = 29,
    #[sys(KI_S)]
    S = 30,
    #[sys(KI_T)]
    T = 31,
    #[sys(KI_U)]
    U = 32,
    #[sys(KI_V)]
    V = 33,
    #[sys(KI_W)]
    W = 34,
    #[sys(KI_X)]
    X = 35,
    #[sys(KI_Y)]
    Y = 36,
    #[sys(KI_Z)]
    Z = 37,
    #[sys(KI_OEM_1)]
    Oem1 = 38,
    #[sys(KI_OEM_PLUS)]
    OemPlus = 39,
    #[sys(KI_OEM_COMMA)]
    OemComma = 40,
    #[sys(KI_OEM_MINUS)]
    OemMinus = 41,
    #[sys(KI_OEM_PERIOD)]
    OemPeriod = 42,
    #[sys(KI_OEM_2)]
    Oem2 = 43,
    #[sys(KI_OEM_3)]
    Oem3 = 44,
    #[sys(KI_OEM_4)]
    Oem4 = 45,
    #[sys(KI_OEM_5)]
    Oem5 = 46,
    #[sys(KI_OEM_6)]
    Oem6 = 47,
    #[sys(KI_OEM_7)]
    Oem7 = 48,
    #[sys(KI_OEM_8)]
    Oem8 = 49,
    #[sys(KI_OEM_102)]
    Oem102 = 50,
    #[sys(KI_NUMPAD0)]
    Numpad0 = 51,
    #[sys(KI_NUMPAD1)]
    Numpad1 = 52,
    #[sys(KI_NUMPAD2)]
    Numpad2 = 53,
    #[sys(KI_NUMPAD3)]
    Numpad3 = 54,
    #[sys(KI_NUMPAD4)]
    Numpad4 = 55,
    #[sys(KI_NUMPAD5)]
    Numpad5 = 56,
    #[sys(KI_NUMPAD6)]
    Numpad6 = 57,
    #[sys(KI_NUMPAD7)]
    Numpad7 = 58,
    #[sys(KI_NUMPAD8)]
    Numpad8 = 59,
    #[sys(KI_NUMPAD9)]
    Numpad9 = 60,
    #[sys(KI_NUMPADENTER)]
    NumpadEnter = 61,
    #[sys(KI_MULTIPLY)]
    Multiply = 62,
    #[sys(KI_ADD)]
    Add = 63,
    #[sys(KI_SEPARATOR)]
    Separator = 64,
    #[sys(KI_SUBTRACT)]
    Subtract = 65,
    #[sys(KI_DECIMAL)]
    Decimal = 66,
    #[sys(KI_DIVIDE)]
    Divide = 67,
    #[sys(KI_OEM_NEC_EQUAL)]
    OemNecEqual = 68,
    #[sys(KI_BACK)]
    Back = 69,
    #[sys(KI_TAB)]
    Tab = 70,
    #[sys(KI_CLEAR)]
    Clear = 71,
    #[sys(KI_RETURN)]
    Return = 72,
    #[sys(KI_PAUSE)]
    Pause = 73,
    #[sys(KI_CAPITAL)]
    Capital = 74,
    #[sys(KI_KANA)]
    Kana = 75,
    #[sys(KI_HANGUL)]
    Hangul = 76,
    #[sys(KI_JUNJA)]
    Junja = 77,
    #[sys(KI_FINAL)]
    Final = 78,
    #[sys(KI_HANJA)]
    Hanja = 79,
    #[sys(KI_KANJI)]
    Kanji = 80,
    #[sys(KI_ESCAPE)]
    Escape = 81,
    #[sys(KI_CONVERT)]
    Convert = 82,
    #[sys(KI_NONCONVERT)]
    NonConvert = 83,
    #[sys(KI_ACCEPT)]
    Accept = 84,
    #[sys(KI_MODECHANGE)]
    ModeChange = 85,
    #[sys(KI_PRIOR)]
    Prior = 86,
    #[sys(KI_NEXT)]
    Next = 87,
    #[sys(KI_END)]
    End = 88,
    #[sys(KI_HOME)]
    Home = 89,
    #[sys(KI_LEFT)]
    Left = 90,
    #[sys(KI_UP)]
    Up = 91,
    #[sys(KI_RIGHT)]
    Right = 92,
    #[sys(KI_DOWN)]
    Down = 93,
    #[sys(KI_SELECT)]
    Select = 94,
    #[sys(KI_PRINT)]
    Print = 95,
    #[sys(KI_EXECUTE)]
    Execute = 96,
    #[sys(KI_SNAPSHOT)]
    Snapshot = 97,
    #[sys(KI_INSERT)]
    Insert = 98,
    #[sys(KI_DELETE)]
    Delete = 99,
    #[sys(KI_HELP)]
    Help = 100,
    #[sys(KI_LWIN)]
    LWin = 101,
    #[sys(KI_RWIN)]
    RWin = 102,
    #[sys(KI_APPS)]
    Apps = 103,
    #[sys(KI_POWER)]
    Power = 104,
    #[sys(KI_SLEEP)]
    Sleep = 105,
    #[sys(KI_WAKE)]
    Wake = 106,
    #[sys(KI_F1)]
    F1 = 107,
    #[sys(KI_F2)]
    F2 = 108,
    #[sys(KI_F3)]
    F3 = 109,
    #[sys(KI_F4)]
    F4 = 110,
    #[sys(KI_F5)]
    F5 = 111,
    #[sys(KI_F6)]
    F6 = 112,
    #[sys(KI_F7)]
    F7 = 113,
    #[sys(KI_F8)]
    F8 = 114,
    #[sys(KI_F9)]
    F9 = 115,
    #[sys(KI_F10)]
    F10 = 116,
    #[sys(KI_F11)]
    F11 = 117,
    #[sys(KI_F12)]
    F12 = 118,
    #[sys(KI_F13)]
    F13 = 119,
    #[sys(KI_F14)]
    F14 = 120,
    #[sys(KI_F15)]
    F15 = 121,
    #[sys(KI_F16)]
    F16 = 122,
    #[sys(KI_F17)]
    F17 = 123,
    #[sys(KI_F18)]
    F18 = 124,
    #[sys(KI_F19)]
    F19 = 125,
    #[sys(KI_F20)]
    F20 = 126,
    #[sys(KI_F21)]
    F21 = 127,
    #[sys(KI_F22)]
    F22 = 128,
    #[sys(KI_F23)]
    F23 = 129,
    #[sys(KI_F24)]
    F24 = 130,
    #[sys(KI_NUMLOCK)]
    NumLock = 131,
    #[sys(KI_SCROLL)]
    Scroll = 132,
    #[sys(KI_OEM_FJ_JISHO)]
    OemFjJisho = 133,
    #[sys(KI_OEM_FJ_MASSHOU)]
    OemFjMasshou = 134,
    #[sys(KI_OEM_FJ_TOUROKU)]
    OemFjTouroku = 135,
    #[sys(KI_OEM_FJ_LOYA)]
    OemFjLoya = 136,
    #[sys(KI_OEM_FJ_ROYA)]
    OemFjRoya = 137,
    #[sys(KI_LSHIFT)]
    LShift = 138,
    #[sys(KI_RSHIFT)]
    RShift = 139,
    #[sys(KI_LCONTROL)]
    LControl = 140,
    #[sys(KI_RCONTROL)]
    RControl = 141,
    #[sys(KI_LMENU)]
    LMenu = 142,
    #[sys(KI_RMENU)]
    RMenu = 143,
    #[sys(KI_BROWSER_BACK)]
    BrowserBack = 144,
    #[sys(KI_BROWSER_FORWARD)]
    BrowserForward = 145,
    #[sys(KI_BROWSER_REFRESH)]
    BrowserRefresh = 146,
    #[sys(KI_BROWSER_STOP)]
    BrowserStop = 147,
    #[sys(KI_BROWSER_SEARCH)]
    BrowserSearch = 148,
    #[sys(KI_BROWSER_FAVORITES)]
    BrowserFavorites = 149,
    #[sys(KI_BROWSER_HOME)]
    BrowserHome = 150,
    #[sys(KI_VOLUME_MUTE)]
    VolumeMute = 151,
    #[sys(KI_VOLUME_DOWN)]
    VolumeDown = 152,
    #[sys(KI_VOLUME_UP)]
    VolumeUp = 153,
    #[sys(KI_MEDIA_NEXT_TRACK)]
    MediaNextTrack = 154,
    #[sys(KI_MEDIA_PREV_TRACK)]
    MediaPrevTrack = 155,
    #[sys(KI_MEDIA_STOP)]
    MediaStop = 156,
    #[sys(KI_MEDIA_PLAY_PAUSE)]
    MediaPlayPause = 157,
    #[sys(KI_LAUNCH_MAIL)]
    LaunchMail = 158,
    #[sys(KI_LAUNCH_MEDIA_SELECT)]
    LaunchMediaSelect = 159,
    #[sys(KI_LAUNCH_APP1)]
    LaunchApp1 = 160,
    #[sys(KI_LAUNCH_APP2)]
    LaunchApp2 = 161,
    #[sys(KI_OEM_AX)]
    OemAx = 162,
    #[sys(KI_ATTN)]
    Attn = 167,
    #[sys(KI_CRSEL)]
    CrSel = 168,
    #[sys(KI_EXSEL)]
    ExSel = 169,
    #[sys(KI_EREOF)]
    ErEof = 170,
    #[sys(KI_PLAY)]
    Play = 171,
    #[sys(KI_ZOOM)]
    Zoom = 172,
    #[sys(KI_PA1)]
    Pa1 = 173,
    #[sys(KI_OEM_CLEAR)]
    OemClear = 174,
    #[sys(KI_FIRST_CUSTOM_KEY)]
    FirstCustomKey = 177,
}

#[sys_cast(bitflags(from = Rml_Input_KeyModifier, repr = u8))]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct KeyModifier: u8 {
        #[sys(KM_CTRL)]
        const CTRL        = 1;
        #[sys(KM_SHIFT)]
        const SHIFT       = 2;
        #[sys(KM_ALT)]
        const ALT         = 4;
        #[sys(KM_META)]
        const META        = 8;
        #[sys(KM_CAPSLOCK)]
        const CAPS_LOCK   = 16;
        #[sys(KM_NUMLOCK)]
        const NUM_LOCK    = 32;
        #[sys(KM_SCROLLLOCK)]
        const SCROLL_LOCK = 64;
    }
}
