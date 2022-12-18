

// HowTo - Set a Winforms parent to a WPF control
// Ref - http://msdn.microsoft.com/en-us/library/system.windows.interop.windowinterophelper(v=vs.110).aspx

var selectCellView = new WPFView();
var helper = new WindowInteropHelper ( selectCellView );
helper.Owner = Application .MainWindow. Handle;

