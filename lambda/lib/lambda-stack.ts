import * as apigw from "@aws-cdk/aws-apigatewayv2";
import { CorsHttpMethod, HttpMethod } from "@aws-cdk/aws-apigatewayv2";
import * as intg from "@aws-cdk/aws-apigatewayv2-integrations";
import * as lambda from "@aws-cdk/aws-lambda";
import * as cdk from "@aws-cdk/core";

export class LambdaStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const glitchHandler = new lambda.Function(this, "GlitchHandler", {
      code: lambda.Code.fromAsset(
        "../artifacts"
      ),
      handler: "unrelated",
      runtime: lambda.Runtime.PROVIDED_AL2,
    });

    const glitchApi = new apigw.HttpApi(this, "GlitchAPI", {
      description: "Image glitching API",
      corsPreflight: {
        allowOrigins: ["*"],
        allowMethods: [CorsHttpMethod.GET]
      },
      defaultIntegration: new intg.LambdaProxyIntegration({
        handler: glitchHandler,
      }),
    });

    new cdk.CfnOutput(this, "glitchApi", {
      value: glitchApi.url!,
    });
  }
}
