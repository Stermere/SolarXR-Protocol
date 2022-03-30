// automatically generated by the FlatBuffers compiler, do not modify

package slimevr_protocol.firmware;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class ConfigureSensorReporting extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static ConfigureSensorReporting getRootAsConfigureSensorReporting(ByteBuffer _bb) { return getRootAsConfigureSensorReporting(_bb, new ConfigureSensorReporting()); }
  public static ConfigureSensorReporting getRootAsConfigureSensorReporting(ByteBuffer _bb, ConfigureSensorReporting obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public ConfigureSensorReporting __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public boolean orientation() { int o = __offset(4); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean rawTransAccel() { int o = __offset(6); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean rawRotVel() { int o = __offset(8); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }

  public static int createConfigureSensorReporting(FlatBufferBuilder builder,
      boolean orientation,
      boolean rawTransAccel,
      boolean rawRotVel) {
    builder.startTable(3);
    ConfigureSensorReporting.addRawRotVel(builder, rawRotVel);
    ConfigureSensorReporting.addRawTransAccel(builder, rawTransAccel);
    ConfigureSensorReporting.addOrientation(builder, orientation);
    return ConfigureSensorReporting.endConfigureSensorReporting(builder);
  }

  public static void startConfigureSensorReporting(FlatBufferBuilder builder) { builder.startTable(3); }
  public static void addOrientation(FlatBufferBuilder builder, boolean orientation) { builder.addBoolean(0, orientation, false); }
  public static void addRawTransAccel(FlatBufferBuilder builder, boolean rawTransAccel) { builder.addBoolean(1, rawTransAccel, false); }
  public static void addRawRotVel(FlatBufferBuilder builder, boolean rawRotVel) { builder.addBoolean(2, rawRotVel, false); }
  public static int endConfigureSensorReporting(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public ConfigureSensorReporting get(int j) { return get(new ConfigureSensorReporting(), j); }
    public ConfigureSensorReporting get(ConfigureSensorReporting obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
}

