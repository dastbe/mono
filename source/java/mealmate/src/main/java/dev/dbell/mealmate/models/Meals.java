package dev.dbell.mealmate.models;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import com.google.common.base.Throwables;
import com.google.common.io.Resources;
import lombok.RequiredArgsConstructor;
import lombok.Value;
import lombok.Builder;
import lombok.extern.jackson.Jacksonized;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.Map;

@Value
@Builder
@Jacksonized
public class Meals {
    private static final ObjectMapper MAPPER = new ObjectMapper(new YAMLFactory());
    private static final TypeReference<HashMap<String, Meals>> MEALS_MAP_TYPE = new TypeReference<HashMap<String, Meals>>() {};

    private final String breakfast;
    private final String lunch;
    private final String dinner;

    public static Map<String, Meals> loadAll() {
        try {
            return MAPPER.readValue(Resources.toString(Resources.getResource("calendar.yml"), StandardCharsets.UTF_8), MEALS_MAP_TYPE);
        } catch (IOException e) {
            Throwables.propagate(e);
        }

        // unreachable
        return null;
    }
}
