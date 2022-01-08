import { App } from 'aws-cdk-lib'
import { HelloStack } from './stack'

const app = new App()

new HelloStack(app, 'HelloStack', {
  env: {
    account: '270025496640',
    region: 'us-west-2'
  }
})
app.synth()
