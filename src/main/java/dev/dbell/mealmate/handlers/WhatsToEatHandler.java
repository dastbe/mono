package dev.dbell.mealmate.handlers;

import com.amazon.ask.dispatcher.request.handler.HandlerInput;
import com.amazon.ask.dispatcher.request.handler.impl.IntentRequestHandler;
import com.amazon.ask.model.IntentRequest;
import com.amazon.ask.model.Response;
import com.amazon.ask.model.Slot;
import dev.dbell.mealmate.models.Meals;
import lombok.extern.slf4j.Slf4j;

import java.time.ZonedDateTime;
import java.util.Map;
import java.util.Optional;

import static com.amazon.ask.request.Predicates.intentName;

@Slf4j
public class WhatsToEatHandler implements IntentRequestHandler {
    @Override
    public boolean canHandle(HandlerInput input, IntentRequest intentRequest) {
        return input.matches(intentName("WhatsToEatIntent"));
    }

    @Override
    public Optional<Response> handle(HandlerInput input, IntentRequest intentRequest) {
        final Slot s = intentRequest.getIntent()
                .getSlots()
                .get("Date");

        final String date = s.getValue() == null ? "today" : s.getValue();

        ZonedDateTime lookupDate = ZonedDateTime.now();
        if (date.equalsIgnoreCase("yesterday")) {
            lookupDate = lookupDate.minusDays(1);
        } else if (date.equalsIgnoreCase("tomorrow")) {
            lookupDate = lookupDate.plusDays(1);
        }

        Map<String, Meals> meals = Meals.loadAll();
        final String lookupKey = String.format("%s-%s-%s", lookupDate.getYear(), lookupDate.getMonth().getValue(), lookupDate.getDayOfMonth());
        log.info("Searching date={} with lookupKey={}", date, lookupKey);

        Meals m = meals.get(lookupKey);
        if (m == null) {
            return input.getResponseBuilder()
                    .withSpeech(String.format("Sorry, I don't have any meals for %s", date))
                    .withShouldEndSession(true)
                    .build();
        }

        final String summary = String.format("%s for breakfast you will have %s, for lunch you will have %s, and for dinner you will have %s",
                date,
                m.getBreakfast(),
                m.getLunch(),
                m.getDinner());

        return input.getResponseBuilder()
                .withSimpleCard("Meal Mate", summary)
                .withSpeech(summary)
                .withShouldEndSession(true)
                .build();
    }
}
