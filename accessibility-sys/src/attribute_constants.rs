#![allow(non_upper_case_globals)]
pub const kAXRoleAttribute: &str = "AXRole";
pub const kAXSubroleAttribute: &str = "AXSubrole";
pub const kAXRoleDescriptionAttribute: &str = "AXRoleDescription";
pub const kAXHelpAttribute: &str = "AXHelp";
pub const kAXTitleAttribute: &str = "AXTitle";
pub const kAXValueAttribute: &str = "AXValue";
pub const kAXValueDescriptionAttribute: &str = "AXValueDescription";
pub const kAXMinValueAttribute: &str = "AXMinValue";
pub const kAXMaxValueAttribute: &str = "AXMaxValue";
pub const kAXValueIncrementAttribute: &str = "AXValueIncrement";
pub const kAXAllowedValuesAttribute: &str = "AXAllowedValues";
pub const kAXPlaceholderValueAttribute: &str = "AXPlaceholderValue";
pub const kAXEnabledAttribute: &str = "AXEnabled";
pub const kAXElementBusyAttribute: &str = "AXElementBusy";
pub const kAXFocusedAttribute: &str = "AXFocused";
pub const kAXParentAttribute: &str = "AXParent";
pub const kAXChildrenAttribute: &str = "AXChildren";
pub const kAXSelectedChildrenAttribute: &str = "AXSelectedChildren";
pub const kAXVisibleChildrenAttribute: &str = "AXVisibleChildren";
pub const kAXWindowAttribute: &str = "AXWindow";
pub const kAXTopLevelUIElementAttribute: &str = "AXTopLevelUIElement";
pub const kAXPositionAttribute: &str = "AXPosition";
pub const kAXSizeAttribute: &str = "AXSize";
pub const kAXOrientationAttribute: &str = "AXOrientation";
pub const kAXDescriptionAttribute: &str = "AXDescription";
pub const kAXDescription: &str = "AXDescription"; // old name
pub const kAXSelectedTextAttribute: &str = "AXSelectedText";
pub const kAXSelectedTextRangeAttribute: &str = "AXSelectedTextRange";
pub const kAXSelectedTextRangesAttribute: &str = "AXSelectedTextRanges";
pub const kAXVisibleCharacterRangeAttribute: &str = "AXVisibleCharacterRange";
pub const kAXNumberOfCharactersAttribute: &str = "AXNumberOfCharacters";
pub const kAXSharedTextUIElementsAttribute: &str = "AXSharedTextUIElements";
pub const kAXSharedCharacterRangeAttribute: &str = "AXSharedCharacterRange";
pub const kAXSharedFocusElementsAttribute: &str = "AXSharedFocusElements";
pub const kAXInsertionPointLineNumberAttribute: &str = "AXInsertionPointLineNumber";
pub const kAXMainAttribute: &str = "AXMain";
pub const kAXMinimizedAttribute: &str = "AXMinimized";
pub const kAXCloseButtonAttribute: &str = "AXCloseButton";
pub const kAXZoomButtonAttribute: &str = "AXZoomButton";
pub const kAXMinimizeButtonAttribute: &str = "AXMinimizeButton";
pub const kAXToolbarButtonAttribute: &str = "AXToolbarButton";
pub const kAXFullScreenButtonAttribute: &str = "AXFullScreenButton";
pub const kAXProxyAttribute: &str = "AXProxy";
pub const kAXGrowAreaAttribute: &str = "AXGrowArea";
pub const kAXModalAttribute: &str = "AXModal";
pub const kAXDefaultButtonAttribute: &str = "AXDefaultButton";
pub const kAXCancelButtonAttribute: &str = "AXCancelButton";
pub const kAXMenuItemCmdCharAttribute: &str = "AXMenuItemCmdChar";
pub const kAXMenuItemCmdVirtualKeyAttribute: &str = "AXMenuItemCmdVirtualKey";
pub const kAXMenuItemCmdGlyphAttribute: &str = "AXMenuItemCmdGlyph";
pub const kAXMenuItemCmdModifiersAttribute: &str = "AXMenuItemCmdModifiers";
pub const kAXMenuItemMarkCharAttribute: &str = "AXMenuItemMarkChar";
pub const kAXMenuItemPrimaryUIElementAttribute: &str = "AXMenuItemPrimaryUIElement";

pub const kAXMenuItemModifierNone: u32 = 0;
pub const kAXMenuItemModifierShift: u32 = 1 << 0;
pub const kAXMenuItemModifierOption: u32 = 1 << 1;
pub const kAXMenuItemModifierControl: u32 = 1 << 2;
pub const kAXMenuItemModifierNoCommand: u32 = 1 << 3;

pub const kAXMenuBarAttribute: &str = "AXMenuBar";
pub const kAXWindowsAttribute: &str = "AXWindows";
pub const kAXFrontmostAttribute: &str = "AXFrontmost";
pub const kAXHiddenAttribute: &str = "AXHidden";
pub const kAXMainWindowAttribute: &str = "AXMainWindow";
pub const kAXFocusedWindowAttribute: &str = "AXFocusedWindow";
pub const kAXFocusedUIElementAttribute: &str = "AXFocusedUIElement";
pub const kAXExtrasMenuBarAttribute: &str = "AXExtrasMenuBar";
pub const kAXHeaderAttribute: &str = "AXHeader";
pub const kAXEditedAttribute: &str = "AXEdited";
pub const kAXValueWrapsAttribute: &str = "AXValueWraps";
pub const kAXTabsAttribute: &str = "AXTabs";
pub const kAXTitleUIElementAttribute: &str = "AXTitleUIElement";
pub const kAXHorizontalScrollBarAttribute: &str = "AXHorizontalScrollBar";
pub const kAXVerticalScrollBarAttribute: &str = "AXVerticalScrollBar";
pub const kAXOverflowButtonAttribute: &str = "AXOverflowButton";
pub const kAXFilenameAttribute: &str = "AXFilename";
pub const kAXExpandedAttribute: &str = "AXExpanded";
pub const kAXSelectedAttribute: &str = "AXSelected";
pub const kAXSplittersAttribute: &str = "AXSplitters";
pub const kAXNextContentsAttribute: &str = "AXNextContents";
pub const kAXDocumentAttribute: &str = "AXDocument";
pub const kAXDecrementButtonAttribute: &str = "AXDecrementButton";
pub const kAXIncrementButtonAttribute: &str = "AXIncrementButton";
pub const kAXPreviousContentsAttribute: &str = "AXPreviousContents";
pub const kAXContentsAttribute: &str = "AXContents";
pub const kAXIncrementorAttribute: &str = "AXIncrementor";
pub const kAXHourFieldAttribute: &str = "AXHourField";
pub const kAXMinuteFieldAttribute: &str = "AXMinuteField";
pub const kAXSecondFieldAttribute: &str = "AXSecondField";
pub const kAXAMPMFieldAttribute: &str = "AXAMPMField";
pub const kAXDayFieldAttribute: &str = "AXDayField";
pub const kAXMonthFieldAttribute: &str = "AXMonthField";
pub const kAXYearFieldAttribute: &str = "AXYearField";
pub const kAXColumnTitleAttribute: &str = "AXColumnTitles";
pub const kAXURLAttribute: &str = "AXURL";
pub const kAXLabelUIElementsAttribute: &str = "AXLabelUIElements";
pub const kAXLabelValueAttribute: &str = "AXLabelValue";
pub const kAXShownMenuUIElementAttribute: &str = "AXShownMenuUIElement";
pub const kAXServesAsTitleForUIElementsAttribute: &str = "AXServesAsTitleForUIElements";
pub const kAXLinkedUIElementsAttribute: &str = "AXLinkedUIElements";
pub const kAXRowsAttribute: &str = "AXRows";
pub const kAXVisibleRowsAttribute: &str = "AXVisibleRows";
pub const kAXSelectedRowsAttribute: &str = "AXSelectedRows";
pub const kAXColumnsAttribute: &str = "AXColumns";
pub const kAXVisibleColumnsAttribute: &str = "AXVisibleColumns";
pub const kAXSelectedColumnsAttribute: &str = "AXSelectedColumns";
pub const kAXSortDirectionAttribute: &str = "AXSortDirection";
pub const kAXIndexAttribute: &str = "AXIndex";
pub const kAXDisclosingAttribute: &str = "AXDisclosing";
pub const kAXDisclosedRowsAttribute: &str = "AXDisclosedRows";
pub const kAXDisclosedByRowAttribute: &str = "AXDisclosedByRow";
pub const kAXDisclosureLevelAttribute: &str = "AXDisclosureLevel";
pub const kAXMatteHoleAttribute: &str = "AXMatteHole";
pub const kAXMatteContentUIElementAttribute: &str = "AXMatteContentUIElement";
pub const kAXMarkerUIElementsAttribute: &str = "AXMarkerUIElements";
pub const kAXUnitsAttribute: &str = "AXUnits";
pub const kAXUnitDescriptionAttribute: &str = "AXUnitDescription";
pub const kAXMarkerTypeAttribute: &str = "AXMarkerType";
pub const kAXMarkerTypeDescriptionAttribute: &str = "AXMarkerTypeDescription";
pub const kAXIsApplicationRunningAttribute: &str = "AXIsApplicationRunning";
pub const kAXSearchButtonAttribute: &str = "AXSearchButton";
pub const kAXClearButtonAttribute: &str = "AXClearButton";
pub const kAXFocusedApplicationAttribute: &str = "AXFocusedApplication";
pub const kAXRowCountAttribute: &str = "AXRowCount";
pub const kAXColumnCountAttribute: &str = "AXColumnCount";
pub const kAXOrderedByRowAttribute: &str = "AXOrderedByRow";
pub const kAXWarningValueAttribute: &str = "AXWarningValue";
pub const kAXCriticalValueAttribute: &str = "AXCriticalValue";
pub const kAXSelectedCellsAttribute: &str = "AXSelectedCells";
pub const kAXVisibleCellsAttribute: &str = "AXVisibleCells";
pub const kAXRowHeaderUIElementsAttribute: &str = "AXRowHeaderUIElements";
pub const kAXColumnHeaderUIElementsAttribute: &str = "AXColumnHeaderUIElements";
pub const kAXRowIndexRangeAttribute: &str = "AXRowIndexRange";
pub const kAXColumnIndexRangeAttribute: &str = "AXColumnIndexRange";
pub const kAXHorizontalUnitsAttribute: &str = "AXHorizontalUnits";
pub const kAXVerticalUnitsAttribute: &str = "AXVerticalUnits";
pub const kAXHorizontalUnitDescriptionAttribute: &str = "AXHorizontalUnitDescription";
pub const kAXVerticalUnitDescriptionAttribute: &str = "AXVerticalUnitDescription";
pub const kAXHandlesAttribute: &str = "AXHandles";
pub const kAXTextAttribute: &str = "AXText";
pub const kAXVisibleTextAttribute: &str = "AXVisibleText";
pub const kAXIsEditableAttribute: &str = "AXIsEditable";
pub const kAXColumnTitlesAttribute: &str = "AXColumnTitles";
pub const kAXIdentifierAttribute: &str = "AXIdentifier";
pub const kAXAlternateUIVisibleAttribute: &str = "AXAlternateUIVisible";
pub const kAXLineForIndexParameterizedAttribute: &str = "AXLineForIndex";
pub const kAXRangeForLineParameterizedAttribute: &str = "AXRangeForLine";
pub const kAXStringForRangeParameterizedAttribute: &str = "AXStringForRange";
pub const kAXRangeForPositionParameterizedAttribute: &str = "AXRangeForPosition";
pub const kAXRangeForIndexParameterizedAttribute: &str = "AXRangeForIndex";
pub const kAXBoundsForRangeParameterizedAttribute: &str = "AXBoundsForRange";
pub const kAXRTFForRangeParameterizedAttribute: &str = "AXRTFForRange";
pub const kAXAttributedStringForRangeParameterizedAttribute: &str = "AXAttributedStringForRange";
pub const kAXStyleRangeForIndexParameterizedAttribute: &str = "AXStyleRangeForIndex";
pub const kAXCellForColumnAndRowParameterizedAttribute: &str = "AXCellForColumnAndRow";
pub const kAXLayoutPointForScreenPointParameterizedAttribute: &str = "AXLayoutPointForScreenPoint";
pub const kAXLayoutSizeForScreenSizeParameterizedAttribute: &str = "AXLayoutSizeForScreenSize";
pub const kAXScreenPointForLayoutPointParameterizedAttribute: &str = "AXScreenPointForLayoutPoint";
pub const kAXScreenSizeForLayoutSizeParameterizedAttribute: &str = "AXScreenSizeForLayoutSize";