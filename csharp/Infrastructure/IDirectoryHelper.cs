using System.Collections.Generic;


namespace Infrastructure
{
    /// <summary>
    /// Various directory services
    /// </summary>
    public interface IDirectoryHelper
    {
        /// <summary>
        /// Cleans the directory.
        /// </summary>
        /// <param name="dirPath">The dir path.</param>
        void CleanDirectory( string dirPath );

        /// <summary>
        /// Gets all files with extensions.
        /// </summary>
        /// <param name="dirPathToSearch"> the root directory path to search files </param>
        /// <param name="listOfExtensions">The list of extensions.</param>
        /// <returns> List of all files with give extension. </returns>
        IEnumerable<string> GetAllFilesWithExtensions( string dirPathToSearch, IEnumerable<string> listOfExtensions );

        /// <summary>
        /// Moves all files to one directory if they are already not in one directory.
        /// </summary>
        /// <param name="dirPathToSearch">The dir path to search the files.</param>
        /// <param name="listOfExtensions">The list of extensions.</param>
        /// <returns> The path to the directory where all files are moved.</returns>
        string MoveAllFilesToOneDirectory( string dirPathToSearch, IEnumerable<string> listOfExtensions );
    }
}
