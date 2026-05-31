//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 425/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk425<F: Float>(t402: F, t4221: F, t1054: F, t1089: F, t1092: F, t245: F, t977: F, t214: F, t410: F, t1038: F, t1060: F, t1061: F, t1063: F, t1072: F, t1078: F, t1116: F, t1121: F, t1124: F, t1127: F, t1128: F, t167: F, t180: F, t403: F, t4101: F, t4103: F, t4106: F, t418: F, t4190: F, t4202: F, t4203: F, t4214: F, t4220: F, t5: F) -> (F, F) {
    let t4222 = t4221 * t402;
    let t4232 = F::cast_from(0.10685e0_f64) * t1054 * t245 * t1089 * t1092;
    let t4233 = t245 * t977;
    let t4237 = t214 * t410;
    let t4244 = t245 * t1038;
    let t4248 = t4101 - t4106 + F::cast_from(0.51947577317044391277e2_f64) * t1127 * t4190 - F::cast_from(6.0_f64) * t1061 * t403 * t1072 + F::cast_from(0.16562821945185185185e-2_f64) * t5 * t4103 * t167 + F::cast_from(0.56968947174242584612e-3_f64) * t5 * t4103 * t180 + F::cast_from(0.10254018858216406658e4_f64) * t4202 * t4203 - t4214 + t4220 + F::cast_from(6.0_f64) * t1078 * t4222 + F::cast_from(0.10274e0_f64) * t1054 * t245 * t1060 * t1063 - t4232 + F::cast_from(0.32530743900905219526e-1_f64) * t1054 * t4233 * t1121 + F::cast_from(0.21687162600603479684e-1_f64) * t1054 * t4237 * t418 - F::cast_from(0.16265371950452609763e-1_f64) * t1054 * t1116 * t1124 - F::cast_from(0.48159733137676571078e0_f64) * t1054 * t4244 * t1128;
    (t4232, t4248)
}
