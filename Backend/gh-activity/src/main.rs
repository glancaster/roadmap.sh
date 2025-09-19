use std::env;
use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), &'static str> {
    // Read in the Github username through arguments
    // First one will always be the path of executable
    // From there on are the arguments passed in
    let args:Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        return Err("Please supply the Github Username");
    }
    // Instantiate a helper class to store relevant data for output
    let mut gh_activity = GithubActivity::new(args[0].to_string());
    gh_activity.fetch_recent_activity();
    gh_activity.display();
    Ok(())
}
#[derive(Default)]
struct GithubActivity {
    user: String,
    events: Vec<Event>,
}
impl GithubActivity {
    fn new(user: String) -> Self {
        GithubActivity {
            user,
            ..Default::default()
        }
    }
    fn fetch_recent_activity(&mut self) {
        let curl_events = Command::new("curl")
            .arg("-s")
            .arg("-L")
            .arg("-H \"Accept: application/vnd.github+json\" ")
            .arg("-H \"X-GitHub-Api-Version: 2022-11-28\" ")
            .arg(format!("https://api.github.com/users/{}/events", self.user).as_str())
            .output()
            .expect("failed to execute process");
        if curl_events.status.success() {
            let body: &str = str::from_utf8(&curl_events.stdout).expect("Unable to convert to str::utf8");
            let mut event = Event::new();
            let mut in_repo = false;
            for line in body.lines() {
                match line {
                    // Github Event Types
                    // This could kick off building an event and methods return the updated type
                    // public: true could end the build and append to hashmap/array of events
                    line if line.contains("\"type\"") => {
                        event.process_type(line);
                    },
                    line if line.contains("\"repo\"") => {
                        in_repo = true;
                    },
                    line if line.contains("\"name\"") && in_repo => {
                        event.process_repo(line);
                        in_repo = false;
                    },
                    line if line.contains("\"public\": true") => {
                        //self.process_repo(line); 
                        self.events.push(event);
                        event = Event::new();
                    }
                    _ => {}
                }
            }
        } else {
            eprintln!("Error processing curl");
        }
    }
    fn display(&self) {
        println!("Github Username: {}", self.user);
        if self.events.is_empty() {
            println!("-- No Events Found or Non-Existent User--");
        }
        let mut events_map: HashMap<EventType, Vec<String>> = HashMap::new(); 
        for event in self.events.iter() {
            events_map.entry(event.event_type.expect("must be some event_type")).and_modify(|v| v.push(event.repo.clone().unwrap())).or_insert(vec![event.repo.clone().unwrap()]);
        }
        for (event, values) in events_map.iter() {
            let mut freq_repo_map: HashMap<String, u32> = HashMap::new();
            for repo in values {
                freq_repo_map.entry(repo.clone()).and_modify(|v| *v +=1).or_insert(1);
            }
            match event {
                EventType::WatchEvent => {
                    println!("- Starred {:?} Respositor(y/ies)", values.len());
                    for (repo, freq) in freq_repo_map {
                        println!("\t- ({}) {:?}", freq, repo);
                    }
                },
                EventType::PushEvent => {
                    println!("- Pushed {:?} Commit(s)", values.len());
                    for (repo, freq) in freq_repo_map {
                        println!("\t- ({}) {:?}", freq, repo);
                    }
                },
                EventType::CreateEvent => {
                    println!("- Created {:?} Branch(es)", values.len());
                    for (repo, freq) in freq_repo_map {
                        println!("\t- ({}) {:?}", freq, repo);
                    }
                },
                EventType::DeleteEvent => {
                    println!("- Deleted {:?} Branch(es)", values.len());
                    for (repo, freq) in freq_repo_map {
                        println!("\t- ({}) {:?}", freq, repo);
                    }
                },
                EventType::PullRequestEvent => {
                    println!("- {} Pull Request Activit(y/ies)", values.len());
                    for (repo, freq) in freq_repo_map {
                        println!("\t- ({}) {:?}", freq, repo);
                    }
                },
                EventType::PullRequestReviewEvent => {
                    println!("- {} Pull Request Review Activit(y/ies)", values.len());
                    for (repo, freq) in freq_repo_map {
                        println!("\t- ({}) {:?}", freq, repo);
                    }
                },
                EventType::PullRequestReviewCommentEvent => {
                    println!("- {} Pull Request Review Comment(s)", values.len());
                    for (repo, freq) in freq_repo_map {
                        println!("\t- ({}) {:?}", freq, repo);
                    }
                },
                EventType::IssueCommentEvent => {
                    println!("- {} Issue Comment(s)", values.len());
                    for (repo, freq) in freq_repo_map {
                        println!("\t- ({}) {:?}", freq, repo);
                    }
                },
                EventType::IssuesEvent => {
                    println!("- {} Issue Activit(y/ies)", values.len());
                    for (repo, freq) in freq_repo_map {
                        println!("\t- ({}) {:?}", freq, repo);
                    }
                },
                _ => {
                    println!("- Found {:?} for {:?}", values.len(), event);
                }
            }
        }
    }
}

#[derive(Debug, Default,Clone)]
struct Event {
    event_type: Option<EventType>,
    repo: Option<String>,
}

impl Event {
    fn new() -> Self {
        Event::default()
    }
    fn process_type(&mut self, line: &str) {
        match line {
            line if line.contains("CommitCommentEvent") => {self.event_type = Some(EventType::CommitCommentEvent);},
            line if line.contains("CreateEvent") => {self.event_type = Some(EventType::CreateEvent);},
            line if line.contains("DeleteEvent") => {self.event_type = Some(EventType::DeleteEvent);},
            line if line.contains("ForkEvent") => {self.event_type = Some(EventType::ForkEvent);},
            line if line.contains("GollumEvent") => {self.event_type = Some(EventType::GollumEvent);},
            line if line.contains("IssueCommentEvent") => {self.event_type = Some(EventType::IssueCommentEvent);},
            line if line.contains("IssuesEvent") => {self.event_type = Some(EventType::IssuesEvent);},
            line if line.contains("MemberEvent") => {self.event_type = Some(EventType::MemberEvent);},
            line if line.contains("PublicEvent") => {self.event_type = Some(EventType::PublicEvent);},
            line if line.contains("PullRequestEvent") => {self.event_type = Some(EventType::PullRequestEvent);},
            line if line.contains("PullRequestReviewEvent") => {self.event_type = Some(EventType::PullRequestReviewEvent);},
            line if line.contains("PullRequestReviewCommentEvent") => {self.event_type = Some(EventType::PullRequestReviewCommentEvent);},
            line if line.contains("PullRequestReviewThreadEvent") => {self.event_type = Some(EventType::PullRequestReviewThreadEvent);},
            line if line.contains("PushEvent") => {self.event_type = Some(EventType::PushEvent);},
            line if line.contains("ReleaseEvent") => {self.event_type = Some(EventType::ReleaseEvent);},
            line if line.contains("SponsorshipEvent") => {self.event_type = Some(EventType::SponsorshipEvent);},
            line if line.contains("WatchEvent") => {self.event_type = Some(EventType::WatchEvent);},
            _ => ()
        }
    }
    fn process_repo(&mut self,line: &str) {
        let name: Vec<&str> = line.trim().split(&['\"'][..]).collect();
        self.repo = Some(name[3].to_string());
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum EventType {
    CommitCommentEvent,
    CreateEvent, 
    DeleteEvent,
    ForkEvent,
    GollumEvent, 
    IssueCommentEvent,
    IssuesEvent,
    MemberEvent,
    PublicEvent,
    PullRequestEvent,
    PullRequestReviewEvent,
    PullRequestReviewCommentEvent,
    PullRequestReviewThreadEvent,
    PushEvent,
    ReleaseEvent,
    SponsorshipEvent,
    WatchEvent,
}


#[cfg(test)]
mod tests {

}
