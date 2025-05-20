import { theme } from "./theme";
const DEFAULT_SHADER = `#pragma stage(compute)

#pragma input(float, name=blue, default=0.0, min=0.0, max=1.0)
layout(set=1, binding=0) uniform custom_inputs {
    float blue;
};

#pragma target(name="output_image", screen)
layout(rgba8, set=0, binding=1) uniform writeonly image2D output_image;

layout(local_size_x = 16, local_size_y = 16) in;
void main() {
    ivec2 pixel_coords = ivec2(gl_GlobalInvocationID.xy);
    ivec2 image_size = imageSize(output_image);

    vec2 normalized_coords = vec2(pixel_coords) / vec2(image_size);

    vec4 color = vec4(normalized_coords.x, normalized_coords.y, blue, 1.0);

    vec4 inverse = vec4(vec3(1.0) - color.xyz, 1.0);
    imageStore(output_image, pixel_coords, color);
}
`;

export type WasmInputValue =
  | { type: 'Float'; current: number; min: number; max: number; default: number }
  | { type: 'Int'; current: number; min: number; max: number; default: number; labels?: Array<[string, number]> }
  | { type: 'Point'; current: [number, number]; min: [number, number]; max: [number, number]; default: [number, number] }
  | { type: 'Bool'; current: boolean; default: boolean }
  | { type: 'Color'; current: [number, number, number, number]; default: [number, number, number, number] }
  | { type: 'Image'; status: string; width?: number; height?: number }
  | { type: 'RawBytes'; bytes: Uint8Array };

export type JsInputValue =
  | { value: number; min: number; max: number; _default: number }
  | { value: number; min: number; max: number; _default: number; labels?: Array<[string, number]> }
  | { value: { x: number, y: number }; min: [number, number]; max: [number, number]; _default: [number, number] }
  | { value: boolean; _default: boolean }
  | { value: [number, number, number, number]; _default: [number, number, number, number] }
  | { status: String; width?: number; height?: number }
  | { bytes: Uint8Array };

export function inputValueToProps(input: WasmInputValue): JsInputValue | null {
  switch (input.type) {
    case 'Float':
      return {
        value: input.current,
        min: input.min,
        max: input.max,
        _default: input.default
      };
    case 'Int':
      return {
        value: input.current,
        min: input.min,
        max: input.max,
        _default: input.default,
        labels: input.labels || []
      };
    case 'Point':
      // Convert from [x, y] array to {x, y} object
      return {
        value: { x: input.current[0], y: input.current[1] },
        min: input.min,
        max: input.max,
        _default: input.default
      };
    case 'Bool':
      return {
        value: input.current,
        _default: input.default
      };
    case 'Color':
      return {
        value: input.current,
        _default: input.default
      };
    case 'Image':
      return {
        status: input.status.toString(),
        width: input.width,
        height: input.height
      };
    case 'RawBytes':
      return {
        bytes: input.bytes
      };
    default:
      return null;
  }
}

export { DEFAULT_SHADER, theme }
