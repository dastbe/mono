import * as cdk from 'aws-cdk-lib/core'
import * as ecs from 'aws-cdk-lib/aws-ecs'

export class HelloStack extends cdk.Stack {
    constructor (scope: cdk.App, id: string, props: cdk.StackProps) {
        super(scope, id, props)

        const taskDefinition = new ecs.FargateTaskDefinition(this, 'TaskDef');
        taskDefinition.addContainer('base', {
           image: ecs.ContainerImage.fromTarball('infra/images/example/echo/image.tar'),
        });

        new cdk.CfnOutput(this, "task-def", {
          value: taskDefinition.taskDefinitionArn
        })
    }
}
