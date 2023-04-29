using System.Runtime.InteropServices;
using DateTime = System.DateTime;

namespace cs_hello;
class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine($"Hello from {RuntimeInformation.OSArchitecture} at {DateTime.Now}");
    }
}
