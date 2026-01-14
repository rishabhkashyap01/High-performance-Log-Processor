public class LogProcessor {
    // Declare the native method (implemented in Rust)
    public native int countErrors(String filePath);

    static {
        // Load the Rust library (ensure path is correct or in system path)
        System.loadLibrary("log_processor"); 
    }

    public static void main(String[] args) {
        LogProcessor processor = new LogProcessor();
        String testFile = "app.log"; // Create a dummy file with some "ERROR" lines
        
        int result = processor.countErrors(testFile);
        
        if (result == -1) {
            System.out.println("Could not open file.");
        } else {
            System.out.println("Total Errors Found: " + result);
        }
    }
}