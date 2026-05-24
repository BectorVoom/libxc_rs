//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1141/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1141<F: Float>(t12710: F, t12725: F, t162: F, t189: F, t489: F, t9841: F, t3245: F, t541: F, t1206: F, t12673: F, t12678: F, t12679: F, t12688: F, t12690: F, t12692: F, t1625: F, t198: F, t3183: F, t3184: F, t3387: F, t4478: F, t4524: F, t4525: F, t4528: F, t4532: F, t7929: F, t7932: F, t7936: F, t9839: F, t9844: F, t9846: F, t9848: F, t9854: F) -> (F, F, F, F) {
    let t12727 = (t12710 + t12725) * t162;
    let t12728 = t12727 * t189;
    let t12729 = t489 * t12728;
    let t12730 = F::cast_from(0.21687162600603479684e-1_f64) * t9841;
    let t12731 = t3245 * t541;
    let t12737 = F::new(6.0) * t1206 * t12673 * t3183 - F::new(6.0) * t12679 * t3183 * t4525 + F::new(6.0) * t12731 * t1625 * t198 + F::new(12.0) * t3184 * t4478 * t4532 + F::new(6.0) * t3245 * t4528 * t4532 - t3387 * t4524 * t4525 + t12678 - t12688 - t12690 + t12692 + t12729 + t12730 + t7929 - t7932 - t7936 - t9839 + t9844 + t9846 - t9848 + t9854;
    (t12727, t12729, t12730, t12737)
}
