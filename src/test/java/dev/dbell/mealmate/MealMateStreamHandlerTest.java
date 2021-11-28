package dev.dbell.mealmate;

import com.amazonaws.services.lambda.runtime.ClientContext;
import com.amazonaws.services.lambda.runtime.CognitoIdentity;
import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.LambdaLogger;
import com.fasterxml.jackson.core.JsonFactory;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.google.common.io.Resources;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;

class MealMateStreamHandlerTest {
    private final ObjectMapper MAPPER = new ObjectMapper(new JsonFactory());

    @ParameterizedTest
    @ValueSource(strings = {
            "whats_to_eat_default",
            "whats_to_eat_no_date",
            "whats_to_eat_tomorrow"
    })
    public void testExamples(final String example) throws Exception {
        final InputStream input = new ByteArrayInputStream(
                Resources.toString(Resources.getResource(String.format("inputs/%s.json", example)),
                        StandardCharsets.UTF_8).getBytes());
        final OutputStream output = new ByteArrayOutputStream();


        final MealMateStreamHandler h = new MealMateStreamHandler();
        h.handleRequest(input, output, getContext());

        // pretty print
        // user agent doesn't appear to be straightforward to force, so we edit it afterwards :shrug:
        final JsonNode json = MAPPER.readTree(output.toString());
        ((ObjectNode)json).put("userAgent", "unset");
        final String prettyOutput = json.toPrettyString();

        // compare
        String desiredOutput = Resources.toString(Resources.getResource(String.format("outputs/%s.json", example)), StandardCharsets.UTF_8);
        Assertions.assertEquals(desiredOutput, prettyOutput);
    }

    private Context getContext() {
        return new Context() {
            @Override
            public String getAwsRequestId() {
                return null;
            }

            @Override
            public String getLogGroupName() {
                return null;
            }

            @Override
            public String getLogStreamName() {
                return null;
            }

            @Override
            public String getFunctionName() {
                return null;
            }

            @Override
            public String getFunctionVersion() {
                return null;
            }

            @Override
            public String getInvokedFunctionArn() {
                return null;
            }

            @Override
            public CognitoIdentity getIdentity() {
                return null;
            }

            @Override
            public ClientContext getClientContext() {
                return null;
            }

            @Override
            public int getRemainingTimeInMillis() {
                return 0;
            }

            @Override
            public int getMemoryLimitInMB() {
                return 0;
            }

            @Override
            public LambdaLogger getLogger() {
                return null;
            }
        };
    }
}