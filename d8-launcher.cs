using System;
using System.Diagnostics;

class Program
{
    static int Main(string[] args)
    {
        string bat = Environment.ExpandEnvironmentVariables(
            @"%LOCALAPPDATA%\Android\Sdk\build-tools\35.0.0\d8.bat");

        var p = new Process();
        p.StartInfo.FileName = "cmd.exe";
        p.StartInfo.Arguments = "/c \"" + bat + "\" " + string.Join(" ", args);
        p.StartInfo.UseShellExecute = false;

        p.Start();
        p.WaitForExit();

        return p.ExitCode;
    }
}