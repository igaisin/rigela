/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use serde::{Deserialize, Serialize};
use win_wrap::input::{
    VirtualKey, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_A, VK_ABNT_C1,
    VK_ABNT_C2, VK_ACCEPT, VK_ADD, VK_APPS, VK_ATTN, VK_B, VK_BACK, VK_BROWSER_BACK,
    VK_BROWSER_FAVORITES, VK_BROWSER_FORWARD, VK_BROWSER_HOME, VK_BROWSER_REFRESH,
    VK_BROWSER_SEARCH, VK_BROWSER_STOP, VK_C, VK_CANCEL, VK_CAPITAL, VK_CLEAR, VK_CONTROL,
    VK_CONVERT, VK_CRSEL, VK_D, VK_DBE_ALPHANUMERIC, VK_DBE_CODEINPUT, VK_DBE_DBCSCHAR,
    VK_DBE_DETERMINESTRING, VK_DBE_ENTERDLGCONVERSIONMODE, VK_DBE_ENTERIMECONFIGMODE,
    VK_DBE_FLUSHSTRING, VK_DBE_HIRAGANA, VK_DBE_KATAKANA, VK_DBE_NOCODEINPUT, VK_DBE_ROMAN,
    VK_DBE_SBCSCHAR, VK_DECIMAL, VK_DELETE, VK_DIVIDE, VK_DOWN, VK_E, VK_END, VK_ESCAPE,
    VK_EXECUTE, VK_F, VK_F1, VK_F10, VK_F11, VK_F12, VK_F13, VK_F14, VK_F15, VK_F16, VK_F17,
    VK_F18, VK_F19, VK_F2, VK_F20, VK_F21, VK_F22, VK_F23, VK_F24, VK_F3, VK_F4, VK_F5, VK_F6,
    VK_F7, VK_F8, VK_F9, VK_FINAL, VK_G, VK_GAMEPAD_A, VK_GAMEPAD_B, VK_GAMEPAD_DPAD_DOWN,
    VK_GAMEPAD_DPAD_LEFT, VK_GAMEPAD_DPAD_RIGHT, VK_GAMEPAD_DPAD_UP, VK_GAMEPAD_LEFT_SHOULDER,
    VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON, VK_GAMEPAD_LEFT_THUMBSTICK_DOWN,
    VK_GAMEPAD_LEFT_THUMBSTICK_LEFT, VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT,
    VK_GAMEPAD_LEFT_THUMBSTICK_UP, VK_GAMEPAD_LEFT_TRIGGER, VK_GAMEPAD_MENU,
    VK_GAMEPAD_RIGHT_SHOULDER, VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON,
    VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN, VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT,
    VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT, VK_GAMEPAD_RIGHT_THUMBSTICK_UP, VK_GAMEPAD_RIGHT_TRIGGER,
    VK_GAMEPAD_VIEW, VK_GAMEPAD_X, VK_GAMEPAD_Y, VK_H, VK_HANGEUL, VK_HANJA, VK_HELP, VK_HOME,
    VK_I, VK_ICO_00, VK_ICO_CLEAR, VK_ICO_HELP, VK_IME_OFF, VK_IME_ON, VK_INSERT, VK_J, VK_JUNJA,
    VK_K, VK_L, VK_LAUNCH_APP1, VK_LAUNCH_APP2, VK_LAUNCH_MAIL, VK_LAUNCH_MEDIA_SELECT, VK_LBUTTON,
    VK_LCONTROL, VK_LEFT, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_M, VK_MBUTTON, VK_MEDIA_NEXT_TRACK,
    VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK, VK_MEDIA_STOP, VK_MENU, VK_MODECHANGE, VK_MULTIPLY,
    VK_N, VK_NAVIGATION_ACCEPT, VK_NAVIGATION_CANCEL, VK_NAVIGATION_DOWN, VK_NAVIGATION_LEFT,
    VK_NAVIGATION_MENU, VK_NAVIGATION_RIGHT, VK_NAVIGATION_UP, VK_NAVIGATION_VIEW, VK_NEXT,
    VK_NONCONVERT, VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4,
    VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_O, VK_OEM_1, VK_OEM_102,
    VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_8, VK_OEM_AX, VK_OEM_CLEAR,
    VK_OEM_COMMA, VK_OEM_CUSEL, VK_OEM_FJ_JISHO, VK_OEM_FJ_LOYA, VK_OEM_FJ_MASSHOU, VK_OEM_FJ_ROYA,
    VK_OEM_FJ_TOUROKU, VK_OEM_JUMP, VK_OEM_MINUS, VK_OEM_PA1, VK_OEM_PA2, VK_OEM_PA3,
    VK_OEM_PERIOD, VK_OEM_PLUS, VK_OEM_RESET, VK_OEM_WSCTRL, VK_P, VK_PACKET, VK_PAUSE, VK_PRINT,
    VK_PRIOR, VK_PROCESSKEY, VK_Q, VK_R, VK_RBUTTON, VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU,
    VK_RSHIFT, VK_RWIN, VK_S, VK_SCROLL, VK_SELECT, VK_SEPARATOR, VK_SHIFT, VK_SLEEP, VK_SNAPSHOT,
    VK_SPACE, VK_SUBTRACT, VK_T, VK_TAB, VK_U, VK_UP, VK_V, VK_VOLUME_DOWN, VK_VOLUME_MUTE,
    VK_VOLUME_UP, VK_W, VK_X, VK_XBUTTON1, VK_XBUTTON2, VK_Y, VK_Z,
};

// 特别注意： 命名没有完善， 小键盘 VkNumPad 开头(大写P)， 不要与 VkNumpad (小写p)混淆

/// 键盘枚举
//noinspection SpellCheckingInspection
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum Keys {
    VkRigelA,
    VkNumPad1,
    VkNumPad2,
    VkNumPad3,
    VkNumPad4,
    VkNumPad5,
    VkNumPad6,
    VkNumPad7,
    VkNumPad8,
    VkNumPad9,
    VkNumPadDiv,
    VkNumPadMul,
    VkCtrl,
    Vk0,
    Vk1,
    Vk2,
    Vk3,
    Vk4,
    Vk5,
    Vk6,
    Vk7,
    Vk8,
    Vk9,
    VkA,
    VkAbntC1,
    VkAbntC2,
    VkAccept,
    VkAdd,
    VkApps,
    VkAttn,
    VkB,
    VkBack,
    VkBrowserBack,
    VkBrowserFavorites,
    VkBrowserForward,
    VkBrowserHome,
    VkBrowserRefresh,
    VkBrowserSearch,
    VkBrowserStop,
    VkC,
    VkCancel,
    VkCapital,
    VkClear,
    VkControl,
    VkConvert,
    VkCrsel,
    VkD,
    VkDbeAlphanumeric,
    VkDbeCodeinput,
    VkDbeDbcschar,
    VkDbeDeterminestring,
    VkDbeEnterdlgconversionmode,
    VkDbeEnterimeconfigmode,
    VkDbeFlushstring,
    VkDbeHiragana,
    VkDbeKatakana,
    VkDbeNocodeinput,
    VkDbeRoman,
    VkDbeSbcschar,
    VkDecimal,
    VkDelete,
    VkDivide,
    VkDown,
    VkE,
    VkEnd,
    VkEscape,
    VkExecute,
    VkF,
    VkF1,
    VkF10,
    VkF11,
    VkF12,
    VkF13,
    VkF14,
    VkF15,
    VkF16,
    VkF17,
    VkF18,
    VkF19,
    VkF2,
    VkF20,
    VkF21,
    VkF22,
    VkF23,
    VkF24,
    VkF3,
    VkF4,
    VkF5,
    VkF6,
    VkF7,
    VkF8,
    VkF9,
    VkFinal,
    VkG,
    VkGamepadA,
    VkGamepadB,
    VkGamepadDpadDown,
    VkGamepadDpadLeft,
    VkGamepadDpadRight,
    VkGamepadDpadUp,
    VkGamepadLeftShoulder,
    VkGamepadLeftThumbstickButton,
    VkGamepadLeftThumbstickDown,
    VkGamepadLeftThumbstickLeft,
    VkGamepadLeftThumbstickRight,
    VkGamepadLeftThumbstickUp,
    VkGamepadLeftTrigger,
    VkGamepadMenu,
    VkGamepadRightShoulder,
    VkGamepadRightThumbstickButton,
    VkGamepadRightThumbstickDown,
    VkGamepadRightThumbstickLeft,
    VkGamepadRightThumbstickRight,
    VkGamepadRightThumbstickUp,
    VkGamepadRightTrigger,
    VkGamepadView,
    VkGamepadX,
    VkGamepadY,
    VkH,
    VkHangeul,
    VkHanja,
    VkHelp,
    VkHome,
    VkI,
    VkIco00,
    VkIcoClear,
    VkIcoHelp,
    VkImeOff,
    VkImeOn,
    VkJ,
    VkJunja,
    VkK,
    VkL,
    VkLaunchApp1,
    VkLaunchApp2,
    VkLaunchMail,
    VkLaunchMediaSelect,
    VkLbutton,
    VkLeft,
    VkLmenu,
    VkLshift,
    VkLwin,
    VkM,
    VkMbutton,
    VkMediaNextTrack,
    VkMediaPlayPause,
    VkMediaPrevTrack,
    VkMediaStop,
    VkMenu,
    VkModechange,
    VkMultiply,
    VkN,
    VkNavigationAccept,
    VkNavigationCancel,
    VkNavigationDown,
    VkNavigationLeft,
    VkNavigationMenu,
    VkNavigationRight,
    VkNavigationUp,
    VkNavigationView,
    VkNext,
    VkNonconvert,
    VkNumlock,
    VkNumpad0,
    VkNumpad1,
    VkNumpad2,
    VkNumpad3,
    VkNumpad4,
    VkNumpad5,
    VkNumpad6,
    VkNumpad7,
    VkNumpad8,
    VkNumpad9,
    VkO,
    VkOem1,
    VkOem102,
    VkOem2,
    VkOem3,
    VkOem4,
    VkOem5,
    VkOem6,
    VkOem7,
    VkOem8,
    VkOemAx,
    VkOemClear,
    VkOemComma,
    VkOemCusel,
    VkOemFjJisho,
    VkOemFjLoya,
    VkOemFjMasshou,
    VkOemFjRoya,
    VkOemFjTouroku,
    VkOemJump,
    VkOemMinus,
    VkOemPa1,
    VkOemPa2,
    VkOemPa3,
    VkOemPeriod,
    VkOemPlus,
    VkOemReset,
    VkOemWsctrl,
    VkP,
    VkPacket,
    VkPause,
    VkPrint,
    VkPrior,
    VkProcesskey,
    VkQ,
    VkR,
    VkRbutton,
    VkReturn,
    VkRight,
    VkRmenu,
    VkRshift,
    VkRwin,
    VkS,
    VkScroll,
    VkSelect,
    VkSeparator,
    VkShift,
    VkSleep,
    VkSnapshot,
    VkSpace,
    VkSubtract,
    VkT,
    VkTab,
    VkU,
    VkUp,
    VkV,
    VkVolumeDown,
    VkVolumeMute,
    VkVolumeUp,
    VkW,
    VkX,
    VkXbutton1,
    VkXbutton2,
    VkY,
    VkZ,

    VkNone,
}

impl From<(u32, bool)> for Keys {
    //noinspection SpellCheckingInspection
    fn from(info: (u32, bool)) -> Self {
        let (vk, ext) = (VirtualKey { 0: info.0 as u16 }, info.1);

        match (vk, ext) {
            // 读屏主热键
            (VK_INSERT, false) => Self::VkRigelA,
            (VK_INSERT, true) => Self::VkRigelA,
            (VK_CAPITAL, false) => Self::VkRigelA,

            // 小键盘数字键
            (VK_END, false) => Self::VkNumPad1,
            (VK_DOWN, false) => Self::VkNumPad2,
            (VK_NEXT, false) => Self::VkNumPad3,
            (VK_LEFT, false) => Self::VkNumPad4,
            (VK_CLEAR, false) => Self::VkNumPad5,
            (VK_RIGHT, false) => Self::VkNumPad6,
            (VK_HOME, false) => Self::VkNumPad7,
            (VK_UP, false) => Self::VkNumPad8,
            (VK_PRIOR, false) => Self::VkNumPad9,
            (VK_DIVIDE, true) => Self::VkNumPadDiv,
            (VK_MULTIPLY, false) => Self::VkNumPadMul,

            // Ctrl键
            (VK_LCONTROL, false) => Self::VkCtrl,
            (VK_RCONTROL, true) => Self::VkCtrl,

            (VK_0, false) => Self::Vk0,
            (VK_1, false) => Self::Vk1,
            (VK_2, false) => Self::Vk2,
            (VK_3, false) => Self::Vk3,
            (VK_4, false) => Self::Vk4,
            (VK_5, false) => Self::Vk5,
            (VK_6, false) => Self::Vk6,
            (VK_7, false) => Self::Vk7,
            (VK_8, false) => Self::Vk8,
            (VK_9, false) => Self::Vk9,
            (VK_A, false) => Self::VkA,
            (VK_ABNT_C1, false) => Self::VkAbntC1,
            (VK_ABNT_C2, false) => Self::VkAbntC2,
            (VK_ACCEPT, false) => Self::VkAccept,
            (VK_ADD, false) => Self::VkAdd,
            (VK_APPS, false) => Self::VkApps,
            (VK_ATTN, false) => Self::VkAttn,
            (VK_B, false) => Self::VkB,
            (VK_BACK, false) => Self::VkBack,
            (VK_BROWSER_BACK, false) => Self::VkBrowserBack,
            (VK_BROWSER_FAVORITES, false) => Self::VkBrowserFavorites,
            (VK_BROWSER_FORWARD, false) => Self::VkBrowserForward,
            (VK_BROWSER_HOME, false) => Self::VkBrowserHome,
            (VK_BROWSER_REFRESH, false) => Self::VkBrowserRefresh,
            (VK_BROWSER_SEARCH, false) => Self::VkBrowserSearch,
            (VK_BROWSER_STOP, false) => Self::VkBrowserStop,
            (VK_C, false) => Self::VkC,
            (VK_CANCEL, false) => Self::VkCancel,
            (VK_CLEAR, true) => Self::VkClear,
            (VK_CONTROL, false) => Self::VkControl,
            (VK_CONVERT, false) => Self::VkConvert,
            (VK_CRSEL, false) => Self::VkCrsel,
            (VK_D, false) => Self::VkD,
            (VK_DBE_ALPHANUMERIC, false) => Self::VkDbeAlphanumeric,
            (VK_DBE_CODEINPUT, false) => Self::VkDbeCodeinput,
            (VK_DBE_DBCSCHAR, false) => Self::VkDbeDbcschar,
            (VK_DBE_DETERMINESTRING, false) => Self::VkDbeDeterminestring,
            (VK_DBE_ENTERDLGCONVERSIONMODE, false) => Self::VkDbeEnterdlgconversionmode,
            (VK_DBE_ENTERIMECONFIGMODE, false) => Self::VkDbeEnterimeconfigmode,
            // (VK_DBE_ENTERWORDREGISTERMODE, false) => Self::VkDbeEnterwordregistermode,
            (VK_DBE_FLUSHSTRING, false) => Self::VkDbeFlushstring,
            (VK_DBE_HIRAGANA, false) => Self::VkDbeHiragana,
            (VK_DBE_KATAKANA, false) => Self::VkDbeKatakana,
            (VK_DBE_NOCODEINPUT, false) => Self::VkDbeNocodeinput,
            // (VK_DBE_NOROMAN, false) => Self::VkDbeNoroman,
            (VK_DBE_ROMAN, false) => Self::VkDbeRoman,
            (VK_DBE_SBCSCHAR, false) => Self::VkDbeSbcschar,
            (VK_DECIMAL, false) => Self::VkDecimal,
            (VK_DELETE, false) => Self::VkDelete,
            (VK_DIVIDE, false) => Self::VkDivide,
            (VK_DOWN, true) => Self::VkDown,
            (VK_E, false) => Self::VkE,
            (VK_END, true) => Self::VkEnd,
            // (VK_EREOF, false) => Self::VkEreof,
            (VK_ESCAPE, false) => Self::VkEscape,
            (VK_EXECUTE, false) => Self::VkExecute,
            // (VK_EXSEL, false) => Self::VkExsel,
            (VK_F, false) => Self::VkF,
            (VK_F1, false) => Self::VkF1,
            (VK_F10, false) => Self::VkF10,
            (VK_F11, false) => Self::VkF11,
            (VK_F12, false) => Self::VkF12,
            (VK_F13, false) => Self::VkF13,
            (VK_F14, false) => Self::VkF14,
            (VK_F15, false) => Self::VkF15,
            (VK_F16, false) => Self::VkF16,
            (VK_F17, false) => Self::VkF17,
            (VK_F18, false) => Self::VkF18,
            (VK_F19, false) => Self::VkF19,
            (VK_F2, false) => Self::VkF2,
            (VK_F20, false) => Self::VkF20,
            (VK_F21, false) => Self::VkF21,
            (VK_F22, false) => Self::VkF22,
            (VK_F23, false) => Self::VkF23,
            (VK_F24, false) => Self::VkF24,
            (VK_F3, false) => Self::VkF3,
            (VK_F4, false) => Self::VkF4,
            (VK_F5, false) => Self::VkF5,
            (VK_F6, false) => Self::VkF6,
            (VK_F7, false) => Self::VkF7,
            (VK_F8, false) => Self::VkF8,
            (VK_F9, false) => Self::VkF9,
            (VK_FINAL, false) => Self::VkFinal,
            (VK_G, false) => Self::VkG,
            (VK_GAMEPAD_A, false) => Self::VkGamepadA,
            (VK_GAMEPAD_B, false) => Self::VkGamepadB,
            (VK_GAMEPAD_DPAD_DOWN, false) => Self::VkGamepadDpadDown,
            (VK_GAMEPAD_DPAD_LEFT, false) => Self::VkGamepadDpadLeft,
            (VK_GAMEPAD_DPAD_RIGHT, false) => Self::VkGamepadDpadRight,
            (VK_GAMEPAD_DPAD_UP, false) => Self::VkGamepadDpadUp,
            (VK_GAMEPAD_LEFT_SHOULDER, false) => Self::VkGamepadLeftShoulder,
            (VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON, false) => Self::VkGamepadLeftThumbstickButton,
            (VK_GAMEPAD_LEFT_THUMBSTICK_DOWN, false) => Self::VkGamepadLeftThumbstickDown,
            (VK_GAMEPAD_LEFT_THUMBSTICK_LEFT, false) => Self::VkGamepadLeftThumbstickLeft,
            (VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT, false) => Self::VkGamepadLeftThumbstickRight,
            (VK_GAMEPAD_LEFT_THUMBSTICK_UP, false) => Self::VkGamepadLeftThumbstickUp,
            (VK_GAMEPAD_LEFT_TRIGGER, false) => Self::VkGamepadLeftTrigger,
            (VK_GAMEPAD_MENU, false) => Self::VkGamepadMenu,
            (VK_GAMEPAD_RIGHT_SHOULDER, false) => Self::VkGamepadRightShoulder,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON, false) => Self::VkGamepadRightThumbstickButton,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN, false) => Self::VkGamepadRightThumbstickDown,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT, false) => Self::VkGamepadRightThumbstickLeft,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT, false) => Self::VkGamepadRightThumbstickRight,
            (VK_GAMEPAD_RIGHT_THUMBSTICK_UP, false) => Self::VkGamepadRightThumbstickUp,
            (VK_GAMEPAD_RIGHT_TRIGGER, false) => Self::VkGamepadRightTrigger,
            (VK_GAMEPAD_VIEW, false) => Self::VkGamepadView,
            (VK_GAMEPAD_X, false) => Self::VkGamepadX,
            (VK_GAMEPAD_Y, false) => Self::VkGamepadY,
            (VK_H, false) => Self::VkH,
            (VK_HANGEUL, false) => Self::VkHangeul,
            // (VK_HANGUL, false) => Self::VkHangul,
            (VK_HANJA, false) => Self::VkHanja,
            (VK_HELP, false) => Self::VkHelp,
            (VK_HOME, true) => Self::VkHome,
            (VK_I, false) => Self::VkI,
            (VK_ICO_00, false) => Self::VkIco00,
            (VK_ICO_CLEAR, false) => Self::VkIcoClear,
            (VK_ICO_HELP, false) => Self::VkIcoHelp,
            (VK_IME_OFF, false) => Self::VkImeOff,
            (VK_IME_ON, false) => Self::VkImeOn,
            (VK_J, false) => Self::VkJ,
            (VK_JUNJA, false) => Self::VkJunja,
            (VK_K, false) => Self::VkK,
            // (VK_KANA, false) => Self::VkKana,
            // (VK_KANJI, false) => Self::VkKanji,
            (VK_L, false) => Self::VkL,
            (VK_LAUNCH_APP1, false) => Self::VkLaunchApp1,
            (VK_LAUNCH_APP2, false) => Self::VkLaunchApp2,
            (VK_LAUNCH_MAIL, false) => Self::VkLaunchMail,
            (VK_LAUNCH_MEDIA_SELECT, false) => Self::VkLaunchMediaSelect,
            (VK_LBUTTON, false) => Self::VkLbutton,

            (VK_LEFT, true) => Self::VkLeft,
            (VK_LMENU, false) => Self::VkLmenu,
            (VK_LSHIFT, false) => Self::VkLshift,
            (VK_LWIN, false) => Self::VkLwin,
            (VK_M, false) => Self::VkM,
            (VK_MBUTTON, false) => Self::VkMbutton,
            (VK_MEDIA_NEXT_TRACK, false) => Self::VkMediaNextTrack,
            (VK_MEDIA_PLAY_PAUSE, false) => Self::VkMediaPlayPause,
            (VK_MEDIA_PREV_TRACK, false) => Self::VkMediaPrevTrack,
            (VK_MEDIA_STOP, false) => Self::VkMediaStop,
            (VK_MENU, false) => Self::VkMenu,
            (VK_MODECHANGE, false) => Self::VkModechange,
            (VK_MULTIPLY, true) => Self::VkMultiply,
            (VK_N, false) => Self::VkN,
            (VK_NAVIGATION_ACCEPT, false) => Self::VkNavigationAccept,
            (VK_NAVIGATION_CANCEL, false) => Self::VkNavigationCancel,
            (VK_NAVIGATION_DOWN, false) => Self::VkNavigationDown,
            (VK_NAVIGATION_LEFT, false) => Self::VkNavigationLeft,
            (VK_NAVIGATION_MENU, false) => Self::VkNavigationMenu,
            (VK_NAVIGATION_RIGHT, false) => Self::VkNavigationRight,
            (VK_NAVIGATION_UP, false) => Self::VkNavigationUp,
            (VK_NAVIGATION_VIEW, false) => Self::VkNavigationView,
            (VK_NEXT, true) => Self::VkNext,
            // (VK_NONAME, false) => Self::VkNoname,
            (VK_NONCONVERT, false) => Self::VkNonconvert,
            (VK_NUMLOCK, false) => Self::VkNumlock,
            (VK_NUMPAD0, false) => Self::VkNumpad0,
            (VK_NUMPAD1, false) => Self::VkNumpad1,
            (VK_NUMPAD2, false) => Self::VkNumpad2,
            (VK_NUMPAD3, false) => Self::VkNumpad3,
            (VK_NUMPAD4, false) => Self::VkNumpad4,
            (VK_NUMPAD5, false) => Self::VkNumpad5,
            (VK_NUMPAD6, false) => Self::VkNumpad6,
            (VK_NUMPAD7, false) => Self::VkNumpad7,
            (VK_NUMPAD8, false) => Self::VkNumpad8,
            (VK_NUMPAD9, false) => Self::VkNumpad9,
            (VK_O, false) => Self::VkO,
            (VK_OEM_1, false) => Self::VkOem1,
            (VK_OEM_102, false) => Self::VkOem102,
            (VK_OEM_2, false) => Self::VkOem2,
            (VK_OEM_3, false) => Self::VkOem3,
            (VK_OEM_4, false) => Self::VkOem4,
            (VK_OEM_5, false) => Self::VkOem5,
            (VK_OEM_6, false) => Self::VkOem6,
            (VK_OEM_7, false) => Self::VkOem7,
            (VK_OEM_8, false) => Self::VkOem8,
            // (VK_OEM_ATTN, false) => Self::VkOemAttn,
            // (VK_OEM_AUTO, false) => Self::VkOemAuto,
            (VK_OEM_AX, false) => Self::VkOemAx,
            // (VK_OEM_BACKTAB, false) => Self::VkOemBacktab,
            (VK_OEM_CLEAR, false) => Self::VkOemClear,
            (VK_OEM_COMMA, false) => Self::VkOemComma,
            // (VK_OEM_COPY, false) => Self::VkOemCopy,
            (VK_OEM_CUSEL, false) => Self::VkOemCusel,
            // (VK_OEM_ENLW, false) => Self::VkOemEnlw,
            // (VK_OEM_FINISH, false) => Self::VkOemFinish,
            (VK_OEM_FJ_JISHO, false) => Self::VkOemFjJisho,
            (VK_OEM_FJ_LOYA, false) => Self::VkOemFjLoya,
            (VK_OEM_FJ_MASSHOU, false) => Self::VkOemFjMasshou,
            (VK_OEM_FJ_ROYA, false) => Self::VkOemFjRoya,
            (VK_OEM_FJ_TOUROKU, false) => Self::VkOemFjTouroku,
            (VK_OEM_JUMP, false) => Self::VkOemJump,
            (VK_OEM_MINUS, false) => Self::VkOemMinus,
            // (VK_OEM_NEC_EQUAL, false) => Self::VkOemNecEqual,
            (VK_OEM_PA1, false) => Self::VkOemPa1,
            (VK_OEM_PA2, false) => Self::VkOemPa2,
            (VK_OEM_PA3, false) => Self::VkOemPa3,
            (VK_OEM_PERIOD, false) => Self::VkOemPeriod,
            (VK_OEM_PLUS, false) => Self::VkOemPlus,
            (VK_OEM_RESET, false) => Self::VkOemReset,
            (VK_OEM_WSCTRL, false) => Self::VkOemWsctrl,
            (VK_P, false) => Self::VkP,
            // (VK_PA1, false) => Self::VkPa1,
            (VK_PACKET, false) => Self::VkPacket,
            (VK_PAUSE, false) => Self::VkPause,
            // (VK_PLAY, false) => Self::VkPlay,
            (VK_PRINT, false) => Self::VkPrint,
            (VK_PRIOR, true) => Self::VkPrior,
            (VK_PROCESSKEY, false) => Self::VkProcesskey,
            (VK_Q, false) => Self::VkQ,
            (VK_R, false) => Self::VkR,
            (VK_RBUTTON, false) => Self::VkRbutton,

            (VK_RETURN, false) => Self::VkReturn,
            (VK_RIGHT, true) => Self::VkRight,
            (VK_RMENU, false) => Self::VkRmenu,
            (VK_RSHIFT, false) => Self::VkRshift,
            (VK_RWIN, false) => Self::VkRwin,
            (VK_S, false) => Self::VkS,
            (VK_SCROLL, false) => Self::VkScroll,
            (VK_SELECT, false) => Self::VkSelect,
            (VK_SEPARATOR, false) => Self::VkSeparator,
            (VK_SHIFT, false) => Self::VkShift,
            (VK_SLEEP, false) => Self::VkSleep,
            (VK_SNAPSHOT, false) => Self::VkSnapshot,
            (VK_SPACE, false) => Self::VkSpace,
            (VK_SUBTRACT, false) => Self::VkSubtract,
            (VK_T, false) => Self::VkT,
            (VK_TAB, false) => Self::VkTab,
            (VK_U, false) => Self::VkU,
            (VK_UP, true) => Self::VkUp,
            (VK_V, false) => Self::VkV,
            (VK_VOLUME_DOWN, false) => Self::VkVolumeDown,
            (VK_VOLUME_MUTE, false) => Self::VkVolumeMute,
            (VK_VOLUME_UP, false) => Self::VkVolumeUp,
            (VK_W, false) => Self::VkW,
            (VK_X, false) => Self::VkX,
            (VK_XBUTTON1, false) => Self::VkXbutton1,
            (VK_XBUTTON2, false) => Self::VkXbutton2,
            (VK_Y, false) => Self::VkY,
            (VK_Z, false) => Self::VkZ,
            // (VK_ZOOM, false) => Self::VkZoom,
            _ => Self::VkNone,
        }
    }
}

impl From<&str> for Keys {
    fn from(s: &str) -> Self {
        match s {
            "RigelA" => Self::VkRigelA,
            "0" => Self::Vk0,
            "1" => Self::Vk1,
            "2" => Self::Vk2,
            "3" => Self::Vk3,
            "4" => Self::Vk4,
            "5" => Self::Vk5,
            "6" => Self::Vk6,
            "7" => Self::Vk7,
            "8" => Self::Vk8,
            "9" => Self::Vk9,
            "A" => Self::VkA,
            "B" => Self::VkB,
            "C" => Self::VkC,
            "D" => Self::VkD,
            "E" => Self::VkE,
            "F" => Self::VkF,
            "G" => Self::VkG,
            "H" => Self::VkH,
            "I" => Self::VkI,
            "J" => Self::VkJ,
            "K" => Self::VkK,
            "L" => Self::VkL,
            "M" => Self::VkM,
            "N" => Self::VkN,
            "O" => Self::VkO,
            "P" => Self::VkP,
            "Q" => Self::VkQ,
            "R" => Self::VkR,
            "S" => Self::VkS,
            "T" => Self::VkT,
            "U" => Self::VkU,
            "V" => Self::VkV,
            "W" => Self::VkW,
            "X" => Self::VkX,
            "Y" => Self::VkY,
            "Z" => Self::VkZ,
            _ => Self::VkNone,
        }
    }
}

impl Into<String> for Keys {
    fn into(self) -> String {
        let res = match self {
            Keys::VkRigelA => "RigelA",
            Keys::Vk0 => "0",
            Keys::Vk1 => "1",
            Keys::Vk2 => "2",
            Keys::Vk3 => "3",
            Keys::Vk4 => "4",
            Keys::Vk5 => "5",
            Keys::Vk6 => "6",
            Keys::Vk7 => "7",
            Keys::Vk8 => "8",
            Keys::Vk9 => "9",
            Keys::VkA => "A",
            Keys::VkB => "B",
            Keys::VkC => "C",
            Keys::VkD => "D",
            Keys::VkE => "E",
            Keys::VkF => "F",
            Keys::VkG => "G",
            Keys::VkH => "H",
            Keys::VkI => "I",
            Keys::VkJ => "J",
            Keys::VkK => "K",
            Keys::VkL => "L",
            Keys::VkM => "M",
            Keys::VkN => "N",
            Keys::VkO => "O",
            Keys::VkP => "P",
            Keys::VkQ => "Q",
            Keys::VkR => "R",
            Keys::VkS => "S",
            Keys::VkT => "T",
            Keys::VkU => "U",
            Keys::VkV => "V",
            Keys::VkW => "W",
            Keys::VkX => "X",
            Keys::VkY => "Y",
            Keys::VkZ => "Z",
            Keys::VkNone => "None",
            _ => "NoneKey",
        };
        String::from(res)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_vk_from_num() {
        let key: Keys = (27u32, false).into();
        assert_eq!(key, Keys::VkEscape);
    }

    #[test]
    fn test_vk_from_str() {
        let key: Keys = "A".into();
        assert_eq!(key, Keys::VkA);
    }

    #[test]
    fn test_vk_to_str() {
        let key: Keys = Keys::VkA;
        let key_str: String = key.into();
        assert_eq!(key_str, String::from("A"));
    }
}
