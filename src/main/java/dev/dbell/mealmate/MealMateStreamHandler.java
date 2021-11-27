package dev.dbell.mealmate;

import com.amazon.ask.Skill;
import com.amazon.ask.SkillStreamHandler;
import com.amazon.ask.Skills;
import dev.dbell.mealmate.handlers.CancelandStopIntentHandler;
import dev.dbell.mealmate.handlers.FallbackIntentHandler;
import dev.dbell.mealmate.handlers.HelpIntentHandler;
import dev.dbell.mealmate.handlers.LaunchRequestHandler;
import dev.dbell.mealmate.handlers.SessionEndedRequestHandler;
import dev.dbell.mealmate.handlers.WhatsForXHandler;
import dev.dbell.mealmate.handlers.WhatsToEatHandler;

public class MealMateStreamHandler extends SkillStreamHandler {

    private static Skill getSkill() {
        return Skills.standard()
                .addRequestHandlers(
                        new LaunchRequestHandler(),
                        new CancelandStopIntentHandler(),
                        new SessionEndedRequestHandler(),
                        new HelpIntentHandler(),
                        new FallbackIntentHandler(),
                        new WhatsToEatHandler(),
                        new WhatsForXHandler())
                .build();
    }

    public MealMateStreamHandler() {
        super(getSkill());
    }

}
