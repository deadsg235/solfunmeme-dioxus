use anyhow::Result;

pub trait Workflow: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self) -> Result<()>;
}

pub struct WorkflowManager {
    workflows: Vec<Box<dyn Workflow>>,
}

impl WorkflowManager {
    pub fn new() -> Self {
        WorkflowManager {
            workflows: Vec::new(),
        }
    }

    pub fn register_workflow(&mut self, workflow: Box<dyn Workflow>) {
        self.workflows.push(workflow);
    }

    pub fn execute_workflow(&self, name: &str) -> Result<()> {
        for workflow in &self.workflows {
            if workflow.name() == name {
                return workflow.execute();
            }
        }
        Err(anyhow::anyhow!("Workflow not found: {}", name))
    }
}
