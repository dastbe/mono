import * as s3 from 'aws-cdk-lib/aws-s3'
import * as cdk from 'aws-cdk-lib/core'
import * as ecs from 'aws-cdk-lib/aws-ecs'

export class HelloStack extends cdk.Stack {
    constructor (scope: cdk.App, id: string, props: cdk.StackProps) {
        super(scope, id, props)

        const bucket = new s3.Bucket(this, 'Bucket')

        const taskDefinition = new ecs.FargateTaskDefinition(this, 'TaskDef');
        taskDefinition.addContainer('base', {
           image: ecs.ContainerImage.fromTarball('infra/images/example/echo/image.tar'),
        });

        new cdk.CfnOutput(this, "bucket", {
          value: bucket.bucketArn
        })
    }
}
