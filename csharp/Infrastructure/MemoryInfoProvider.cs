using System ;
using System .Diagnostics;
using System .Management;
using System .Runtime;

// Editing from codehub on iPad
namespace Infrastructure.MemoryInformation
{

    /// <summary>
    /// Implementation of <see cref="TiMemoryInfo"/> .
    /// </summary>
    public class MemoryInfo : IMemoryInfo
    {
        /// <summary>
        /// Instantiates <see cref="TcMemoryInfo"/>. Internals are initialized.
        /// </summary>
        publicMemoryInfo ()
        {
            mRamBytes = 0;
            mIsRamCalculated = false;
        }

        /// <summary>
        /// Get if enough memory is available to create a object.
        /// Background - Large objects (larger than 85 KB) are allocated on the Large Object Heap (LOH). This heap can get
        /// fragmented and when a new object is to be allocated, application can crash. Use this method to know if large
        /// object can be allocated. If this method return false, either wait further till the object can be allocated or
        /// exit the application.
        /// </summary>
        /// <param name="requiredSpaceInMegaBytes"> The size of large object </param>
        /// <returns> true, if enough space is available, otherwise false</returns>
        public bool IsSpaceAvailable( int requiredSpaceInMegaBytes )
        {
            // Using exception handling for flow control can not be avoided here. This is suggested
            // usage by Microsoft. See - http://msdn.microsoft.com/en-us/library/system.runtime.memoryfailpoint.aspx
            bool canCreate = true;
            try
            {
                using( new MemoryFailPoint( requiredSpaceInMegaBytes ) )
                {
                    // Do Nothing
                }
            }
            catch( InsufficientMemoryException )
            {
                canCreate = false;
            }
            return canCreate ;
        }

        /// <summary>
        /// Get if the system memory is low at this moment. This is calculated in terms of free RAM available.
        /// Suggested Criteria - Free RAM less than 20 percent of installed RAM - http://msdn.microsoft.com/en-us/library/ff647791.aspx
        /// </summary>
        /// <param name="percentOfRAM"> Low memory criteria in terms of Percentage of RAM </param>
        /// <exception cref="ArgumentOutOfRangeException"> If the percentage is greater than 100 </exception>
        /// <returns> true if memory is low, else false </returns>
        // ReSharper disable once InconsistentNaming
        public bool IsSystemMemoryLow( uint percentOfRAM )
        {
            if( percentOfRAM > 100 )
            {
                var message = string.Format ( "Percentage provided {0} can not be greater than 100", percentOfRAM );
                throw new ArgumentOutOfRangeException( "percentOfRAM" , message );
            }
            var totalRam = GetTotalRAMInMBytes ();
            var freeRam = GetFreeRAMInMBytes ();
            return ( freeRam * 100 / totalRam ) < percentOfRAM ;
        }

        /// <summary>
        /// Gets RAM occupied by current process
        /// </summary>
        /// <returns> RAM Memory the current process occupies </returns>
        // ReSharper disable once InconsistentNaming
        public double GetRAMShareInMBytesForCurrentProcess()
        {
            return GetRAMShareInMBytesForProcess ( Process.GetCurrentProcess ().Id );
        }

        /// <summary>
        /// Gets RAM occupied by a particular process
        /// </summary>
        /// <param name="processId"> Process ID of which RAM memory is required </param>
        /// <returns> RAM Memory the process occupies </returns>
        /// <exception cref="ArgumentException">
        /// The process specified by the processId parameter is not running. The identifier might be expired.
        /// </exception>
        public double GetRAMShareInMBytesForProcess( int processId )
        {
            var theProcess = Process .GetProcessById( processId );
            return Convert .ToDouble( theProcess .WorkingSet64 ) / BYTE_MEGABYTE_FACTOR ;
        }

        /// <summary>
        /// Gets free RAM available at this moment.
        /// Please note that free RAM fluctuates and the value may not be very accurate
        /// Note - If the RAM is less than 1 MB, this will return 0
        /// </summary>
        /// <returns> Free RAM in Mega Bytes </returns>
        // ReSharper disable once InconsistentNaming
        public double GetFreeRAMInMBytes()
        {
            var ramCounter = new PerformanceCounter ( PERFORMANCE_COUNTER_MEMORY , PERFORMANCE_COUNTER_AVAILABLE_MBYTES );
            return ramCounter .NextValue();
        }

        /// <summary>
        /// Gets RAM of the system in Mega Bytes.
        /// Note - If the RAM is less than 1 MB, this will return 0
        /// </summary>
        /// <returns> RAM in Mega Bytes </returns>
        // ReSharper disable once InconsistentNaming
        public double GetTotalRAMInMBytes()
        {
            if( ! mIsRamCalculated )
            {
                ManagementObjectSearcher theHardwareSearcher =
                new ManagementObjectSearcher ( RAM_QUERY_STRING );
                foreach( var theObject in theHardwareSearcher .Get() )
                {
                    mRamBytes = Convert. ToDouble( theObject[ RAM_PROPERTY ] ) / BYTE_MEGABYTE_FACTOR ;
                }
                mIsRamCalculated = true;
            }
            return mRamBytes ;
        }

        /// <summary>
        /// Used for first time RAM calculation
        /// </summary>
        private bool mIsRamCalculated;

        /// <summary>
        /// Store the calculated RAM
        /// </summary>
        private double mRamBytes;


        /// <summary>
        /// Constant for querying RAM
        /// </summary>
        private const string RAM_QUERY_STRING = "Select * From Win32_ComputerSystem";

        /// <summary>
        /// Constant for retrieving RAM
        /// </summary>
        private const string RAM_PROPERTY = "TotalPhysicalMemory" ;

        /// <summary>
        /// Conversion constant between bytes and megabytes
        /// </summary>
        private const int BYTE_MEGABYTE_FACTOR = 1048576 ;

        /// <summary>
        /// String for "Memory" Performance Counter
        /// </summary>
        private const string PERFORMANCE_COUNTER_MEMORY = "Memory" ;

        /// <summary>
        /// String for "Available Memory in Mega Bytes" Performance Counter
        /// </summary>
        private const string PERFORMANCE_COUNTER_AVAILABLE_MBYTES = "Available MBytes" ;
    }
}
