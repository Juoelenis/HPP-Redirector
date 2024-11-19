#include <iostream>
#include <fstream>
#include <string>
#include <ctime>
#include <sstream>

// Function to get the current time as a string
std::string getCurrentTime() {
    std::time_t now = std::time(0);
    std::tm* localTime = std::localtime(&now);
    std::stringstream timeStream;

    // Format time as [YYYY-MM-DD HH:MM:SS]
    timeStream << (1900 + localTime->tm_year) << "-"
               << (1 + localTime->tm_mon) << "-"
               << localTime->tm_mday << " "
               << localTime->tm_hour << ":"
               << localTime->tm_min << ":"
               << localTime->tm_sec;

    return timeStream.str();
}

// Function to log a message to a file
void logMessage(const std::string& message) {
    std::ofstream logFile("redirector_log.txt", std::ios_base::app);  // Open in append mode
    if (logFile.is_open()) {
        logFile << "[" << getCurrentTime() << "] " << message << std::endl;
        logFile.close();
    } else {
        std::cerr << "Error: Could not open log file!" << std::endl;
    }
}

// Example redirector function (simulated)
void redirectToPage(const std::string& url) {
    logMessage("Redirecting to: " + url);
    // Here you would add the actual redirect code (this is a simulation)
    std::cout << "Redirecting to: " << url << std::endl;

    // For example, handle errors:
    if (url.empty()) {
        logMessage("Error: Empty URL encountered during redirection.");
    }
}

int main() {
    // Log program start
    logMessage("Webpage redirector started.");

    // Simulate some redirections
    redirectToPage("https://www.example.com");
    redirectToPage("https://www.example2.com");

    // Simulate an error
    redirectToPage(""); // This will log an error

    // Log program end
    logMessage("Webpage redirector ended.");

    return 0;
}
