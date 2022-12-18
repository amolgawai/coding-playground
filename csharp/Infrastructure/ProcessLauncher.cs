using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Text;


namespace Infrastructure
{
    /// <summary>
    /// Implementation of the Process launcher interface
    /// </summary>
    public class ProcessLauncher : IProcessLauncher
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="ProcessLauncher"/> class.
        /// If the path is null, default process will be launched with arguments as files
        /// </summary>
        /// <param name="processExePath">The process executable path, can be null </param>
        public ProcessLauncher( string processExePath )
        {
            mProcessExePath = processExePath;
        }

        /// <summary>
        /// Launches the process with specified command line arguments.
        /// A file path can be passed as command line argument
        /// </summary>
        /// <param name="cmdArguments">The command line arguments.</param>
        /// <returns> true, if the process was launched, else false </returns>
        public bool Launch( IEnumerable<string> cmdArguments )
        {
            var theProcess = InternalLaunch( cmdArguments );
            return theProcess != null;
        }

        /// <summary>
        /// Launches a process and waits till it finishes.
        /// </summary>
        /// <param name="cmdArguments">The command arguments.</param>
        /// <param name="waitTime"> Wait time </param>
        /// <returns> The exit code of the process </returns>
        public ProcessLaunchResult LaunchAndWaitToFinish( IEnumerable<string> cmdArguments, TimeSpan waitTime )
        {
            var launchSuccessful = false;
            bool? hasExited = null;
            int? exitCode = null;
            var theProcess = InternalLaunch( cmdArguments );
            if( theProcess != null )
            {
                launchSuccessful = true;
                theProcess.WaitForExit( ( int )waitTime.TotalMilliseconds );
                if( theProcess.HasExited )
                {
                    hasExited = true;
                    exitCode = theProcess.ExitCode;
                }
                else
                {
                    hasExited = false;
                }
            }
            return new ProcessLaunchResult( launchSuccessful, hasExited, exitCode );
        }


        /// <summary>
        /// The process executable path
        /// </summary>
        private readonly string mProcessExePath;

        /// <summary>
        /// The command line separator
        /// </summary>
        private const char CMD_SEPARATOR = ' ';

        /// <summary>
        /// Launch the process by processing path and argument.
        /// </summary>
        /// <param name="cmdArguments">The command arguments.</param>
        /// <returns></returns>
        private Process InternalLaunch( IEnumerable<string> cmdArguments )
        {
            var cmdArgString = CombineCommandLineStrings( cmdArguments );
            return String.IsNullOrEmpty( mProcessExePath ) ? Process.Start( cmdArgString ) : Process.Start( mProcessExePath, cmdArgString );
        }

        /// <summary>
        /// Combines the command line strings.
        /// </summary>
        /// <param name="cmdArguments">The command arguments.</param>
        /// <returns></returns>
        private string CombineCommandLineStrings( IEnumerable<string> cmdArguments )
        {
            var theStringBuilder = new StringBuilder();
            foreach( var cmdArgument in cmdArguments )
            {
                theStringBuilder.Append( cmdArgument );
                theStringBuilder.Append( CMD_SEPARATOR );
            }
            return theStringBuilder.ToString();
        }
    }
}