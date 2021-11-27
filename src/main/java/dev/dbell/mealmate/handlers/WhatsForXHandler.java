package dev.dbell.mealmate.handlers;

import com.amazon.ask.dispatcher.request.handler.HandlerInput;
import com.amazon.ask.dispatcher.request.handler.impl.IntentRequestHandler;
import com.amazon.ask.model.IntentRequest;
import com.amazon.ask.model.Response;
import dev.dbell.mealmate.models.Meals;
import lombok.extern.slf4j.Slf4j;

import java.time.ZonedDateTime;
import java.util.Map;
import java.util.Optional;

import static com.amazon.ask.request.Predicates.intentName;

@Slf4j
public class WhatsForXHandler implements IntentRequestHandler {
    @Override
    public boolean canHandle(HandlerInput input, IntentRequest intentRequest) {
        return input.matches(intentName("WhatsForXIntent"));
    }

    @Override
    public Optional<Response> handle(HandlerInput input, IntentRequest intentRequest) {
        final String mealOfTheDay = intentRequest.getIntent()
                .getSlots()
                .get("Meal")
                .getValue();

        ZonedDateTime lookupDate = ZonedDateTime.now();
        Map<String, Meals> meals = Meals.loadAll();
        final String lookupKey = String.format("%s-%s-%s", lookupDate.getYear(), lookupDate.getMonth().getValue(), lookupDate.getDayOfMonth());
        log.info("Searching lookupKey={}", lookupKey);

        Meals m = meals.get(lookupKey);
        if (m == null) {
            return input.getResponseBuilder()
                    .withSpeech(String.format("Sorry, I don't have any meals for %s", mealOfTheDay))
                    .withShouldEndSession(true)
                    .build();
        }

        String mealSummary;
        switch(mealOfTheDay) {
            case "breakfast":
                mealSummary = m.getBreakfast(); break;
            case "lunch":
                mealSummary = m.getLunch(); break;
            case "dinner":
                mealSummary = m.getDinner(); break;
            default:
                throw new RuntimeException("Invalid meal");
        }

        final String summary = String.format("For %s you will have %s.", mealOfTheDay, mealSummary);
        return input.getResponseBuilder()
                .withSimpleCard("Meal Mate", summary)
                .withSpeech(summary)
                .withShouldEndSession(true)
                .build();
    }
}
