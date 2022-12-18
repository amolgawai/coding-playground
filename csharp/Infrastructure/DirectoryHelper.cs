using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;


namespace Infrastructure
{
    /// <summary>
    /// Various Directory helper functions
    /// </summary>
    class DirectoryHelper : IDirectoryHelper
    {
        /// <summary>
        /// Cleans the directory.
        /// </summary>
        /// <param name="dirPath">The dir path.</param>
        public void CleanDirectory( string dirPath )
        {
            DirectoryInfo theDirectoryInfoInfo = new DirectoryInfo( dirPath );

            foreach( FileInfo file in theDirectoryInfoInfo.GetFiles() )
            {
                file.Delete();
            }
            foreach( DirectoryInfo dir in theDirectoryInfoInfo.GetDirectories() )
            {
                dir.Delete( true );
            }
        }

        /// <summary>
        /// Gets all files with extensions.
        /// </summary>
        /// <param name="dirPathToSearch"> the root directory path to search files </param>
        /// <param name="listOfExtensions">The list of extensions.</param>
        /// <returns> List of all files with give extension. </returns>
        public IEnumerable<string> GetAllFilesWithExtensions( string dirPathToSearch, IEnumerable<string> listOfExtensions )
        {
            return Directory.GetFiles( dirPathToSearch, "*", SearchOption.AllDirectories ).
                Where( file => listOfExtensions.Contains( Path.GetExtension( file ) ) );
        }

        /// <summary>
        /// Moves all files to one directory if they are already not in one directory.
        /// </summary>
        /// <param name="dirPathToSearch">The dir path to search the files.</param>
        /// <param name="listOfExtensions">The list of extensions.</param>
        /// <returns> The path to the directory where all files are moved.</returns>
        public string MoveAllFilesToOneDirectory( string dirPathToSearch, IEnumerable<string> listOfExtensions )
        {
            // ReSharper disable PossibleMultipleEnumeration
            var filesToMove = Directory.GetFiles( dirPathToSearch, "*", SearchOption.AllDirectories ).
                Where( file => listOfExtensions.Contains( Path.GetExtension( file ) ) );
            var resultDirPath = Path.GetDirectoryName( filesToMove.FirstOrDefault() );
            bool createNewDir = filesToMove.Any( file => Path.GetDirectoryName( file ) != resultDirPath );
            if( !createNewDir )
            {
                return resultDirPath;
            }
            resultDirPath = Path.Combine( dirPathToSearch, Guid.NewGuid().ToString() );
            Directory.CreateDirectory( resultDirPath );
            foreach( var file in filesToMove )
            {
                // ReSharper disable once AssignNullToNotNullAttribute
                File.Move( Path.GetFullPath( file ), Path.Combine( resultDirPath, Path.GetFileName( file ) ) );
            }
            // ReSharper restore PossibleMultipleEnumeration
            return resultDirPath;
        }
    }
}
