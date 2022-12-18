using System;


namespace Infrastructure
{
    /// <summary>
    /// Interface for console functions
    /// </summary>
    public interface IConsoleHelper
    {
        /// <summary>
        /// Shows a hint to press any key and waits for console input.
        /// </summary>
        void WaitForAnyKey();

        /// <summary>
        /// Writes the error.
        /// </summary>
        /// <param name="format">The format string.</param>
        /// <param name="arg0">The arguments. </param>
        void WriteError( string format, object arg0 );

        /// <summary>
        /// Writes the error.
        /// </summary>
        /// <param name="message">The format.</param>
        void WriteError( string message );

        /// <summary>
        /// Writes the success.
        /// </summary>
        /// <param name="format">The format string.</param>
        /// <param name="arg0">The arguments. </param>
        void WriteSuccess( string format, object arg0 );

        /// <summary>
        /// Writes the success.
        /// </summary>
        /// <param name="message">The message.</param>
        void WriteSuccess( string message );

        /// <summary>
        /// Writes the exception.
        /// </summary>
        /// <param name="exception">The exception.</param>
        void WriteException( Exception exception );

        /// <summary>
        /// Writes the information.
        /// </summary>
        /// <param name="format">The format string.</param>
        /// <param name="arg0">The arguments. </param>
        void WriteInfo( string format, object arg0 );

        /// <summary>
        /// Writes the information.
        /// </summary>
        /// <param name="message">The message.</param>
        void WriteInfo( string message );

        /// <summary>
        /// Writes the WriteWarning.
        /// </summary>
        /// <param name="format">The format string.</param>
        /// <param name="arg0">The arguments. </param>
        void WriteWarning( string format, object arg0 );

        /// <summary>
        /// Writes the WriteWarning.
        /// </summary>
        /// <param name="message">The message.</param>
        void WriteWarning( string message );

        /// <summary>
        /// Asks yes no question.
        /// </summary>
        /// <param name="question">The Question.</param>
        /// <returns> true if yes, else false </returns>
        bool AskYesNo( string question );
    }
}