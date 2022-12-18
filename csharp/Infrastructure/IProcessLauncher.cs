using System;
using System.Collections.Generic;


namespace Infrastructure
{
    /// <summary>
    /// For launching external processes
    /// Note :- The interfaces from common are too heavy and not really useful for requirement of this project.
    /// </summary>
    public interface IProcessLauncher
    {
        /// <summary>
        /// Launches a new process with specified command line arguments.
        /// A file path can be passed as command line argument
        /// </summary>
        /// <param name="cmdArguments">The command line arguments.</param>
        /// <returns> true, if the process was launched, else false </returns>
        bool Launch( IEnumerable<string> cmdArguments );

        /// <summary>
        /// Launches a process and waits till it finishes.
        /// </summary>
        /// <param name="cmdArguments">The command arguments.</param>
        /// <param name="waitTime"> Wait time </param>
        /// <returns> The result of launching pro </returns>
        ProcessLaunchResult LaunchAndWaitToFinish( IEnumerable<string> cmdArguments, TimeSpan waitTime );

    }

    public struct ProcessLaunchResult
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="ProcessLaunchResult"/> struct.
        /// This is used for indicating the result of launching a process
        /// </summary>
        /// <param name="launchSuccessful">if set to <c>true</c> [launch successful].</param>
        /// <param name="hasExited">The has exited value.</param>
        /// <param name="exitCode">The exit code value.</param>
        public ProcessLaunchResult( bool launchSuccessful, bool? hasExited, int? exitCode )
            : this()
        {
            ExitCode = exitCode;
            HasExited = hasExited;
            LaunchSuccessful = launchSuccessful;
        }

        /// <summary>
        /// Gets a value indicating whether [launch successful].
        /// </summary>
        /// <value>
        ///   <c>true</c> if [launch successful]; otherwise, <c>false</c>.
        /// </value>
        public bool LaunchSuccessful { get; private set; }

        /// <summary>
        /// Gets a value indicating whether the process has exited.
        /// </summary>
        /// <value>
        /// <c>true</c> if the process has exited; otherwise, <c>false</c>.
        /// null if the process was not launched
        /// </value>
        public bool? HasExited { get; private set; }

        /// <summary>
        /// Gets the exit code of process launched.
        /// </summary>
        /// <value>
        /// The exit code. Null if the process was not launched or did not exit
        /// </value>
        public int? ExitCode { get; private set; }
    }
}