using System;
using System.Collections.Generic;


namespace Infrastructure
{
    /// <summary>
    /// Simple class for repeated console interactions
    /// </summary>
    public class ConsoleHelper : IConsoleHelper
    {
        /// <summary>
        /// Shows a hint to press any key and waits for console input.
        /// </summary>
        public void WaitForAnyKey()
        {
            Console.WriteLine( @"Press any key to continue." );
            Console.ReadKey();

        }

        /// <summary>
        /// Writes the error to console.
        /// </summary>
        /// <param name="format">The format string.</param>
        /// <param name="arg0">The arguments. </param>
        public void WriteError( string format, object arg0 )
        {
            Console.ForegroundColor = mColors[ 2 ];
            Console.WriteLine( ERROR + format, arg0 );
        }

        /// <summary>
        /// Writes the error.
        /// </summary>
        /// <param name="message">The format.</param>
        public void WriteError( string message )
        {
            Console.ForegroundColor = mColors[ 2 ];
            Console.WriteLine( ERROR + message );
        }

        /// <summary>
        /// Writes the success.
        /// </summary>
        /// <param name="format">The format string.</param>
        /// <param name="arg0">The arguments. </param>
        public void WriteSuccess( string format, object arg0 )
        {
            Console.ForegroundColor = mColors[ 4 ];
            Console.WriteLine( SUCCESS + format, arg0 );
        }

        /// <summary>
        /// Writes the success.
        /// </summary>
        /// <param name="message">The message.</param>
        public void WriteSuccess( string message )
        {
            Console.ForegroundColor = mColors[ 4 ];
            Console.WriteLine( SUCCESS + message );
        }

        /// <summary>
        /// Writes the exception.
        /// </summary>
        /// <param name="exception">The exception.</param>
        public void WriteException( Exception exception )
        {
            Console.ForegroundColor = mColors[ 1 ];
            Console.WriteLine( exception.ToStringDetailed() );
        }

        /// <summary>
        /// Writes the information.
        /// </summary>
        /// <param name="format">The format string.</param>
        /// <param name="arg0">The arguments. </param>
        public void WriteInfo( string format, object arg0 )
        {
            Console.ForegroundColor = mColors[ 0 ];
            Console.WriteLine( INFO + format, arg0 );
        }

        /// <summary>
        /// Writes the information.
        /// </summary>
        /// <param name="message">The message.</param>
        public void WriteInfo( string message )
        {
            Console.ForegroundColor = mColors[ 0 ];
            Console.WriteLine( INFO + message );
        }

        /// <summary>
        /// Writes the WriteWarning.
        /// </summary>
        /// <param name="format">The format string.</param>
        /// <param name="arg0">The arguments. </param>
        public void WriteWarning( string format, object arg0 )
        {
            Console.ForegroundColor = mColors[ 3 ];
            Console.WriteLine( WARNING + format, arg0 );
        }

        /// <summary>
        /// Writes the WriteWarning.
        /// </summary>
        /// <param name="message">The message.</param>
        public void WriteWarning( string message )
        {
            Console.ForegroundColor = mColors[ 3 ];
            Console.WriteLine( WARNING + message );
        }

        /// <summary>
        /// Asks yes no question.
        /// </summary>
        /// <param name="question">The Question.</param>
        /// <returns> true if yes, else false </returns>
        public bool AskYesNo( string question )
        {
            Console.ForegroundColor = mColors[ 0 ];
            Console.WriteLine( question + "[y/n]" );
            var answer = Console.ReadKey();
            if( answer.Key == ConsoleKey.Y )
            {
                return true;
            }
            return false;
        }

        /// <summary>
        /// The error string
        /// </summary>
        private const string ERROR = "ERROR - ";

        /// <summary>
        /// The success string
        /// </summary>
        private const string SUCCESS = "SUCCESS - ";

        /// <summary>
        /// The information string
        /// </summary>
        private const string INFO = "INFO - ";

        /// <summary>
        /// The information string
        /// </summary>
        private const string WARNING = "WARNING - ";

        /// <summary>
        /// The colors.
        /// </summary>
        private readonly List<ConsoleColor> mColors = new List<ConsoleColor>
        {
                                                          ConsoleColor.White,
                                                          ConsoleColor.DarkRed,
                                                          ConsoleColor.Red,
                                                          ConsoleColor.Yellow,
                                                          ConsoleColor.Green,
                                                      };
    }
}