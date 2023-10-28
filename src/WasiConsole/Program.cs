using System.Runtime.InteropServices;
using DateTime = System.DateTime;

namespace WasiConsole;
class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine($"Hello from {RuntimeInformation.OSArchitecture} at {DateTime.Now}");
    }
}
