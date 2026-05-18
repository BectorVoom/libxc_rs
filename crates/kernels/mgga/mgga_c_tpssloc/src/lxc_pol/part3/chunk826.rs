//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 826/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk826<F: Float>(t1735: F, t248: F, t3570: F, t1213: F, t1009: F, t1720: F, t1011: F, t1212: F, t1226: F, t1730: F, t1174: F, t1218: F, t1227: F, t1232: F, t1737: F, t3506: F, t3515: F, t3536: F, t3577: F, t488: F, t4950: F, t4954: F, t4957: F, t4959: F, t4961: F, t4966: F, t4969: F, t4974: F, t4980: F, t4984: F, t4989: F, t4994: F) -> (F, F, F, F, F) {
    let t4997 = t248 * t3570 * t1735;
    let t4998 = t1213 * t4997;
    let t5000 = t1720 * t1009;
    let t5001 = t5000 * t1011;
    let t5002 = t5001 * t1212;
    let t5005 = t1730 * t1226;
    let t5010 = -t3577 * t4950 / F::new(4608.0) - t3577 * t4954 / F::new(4608.0) + t4957 / F::new(4608.0) - t4959 / F::new(864.0) - t4961 * t488 / F::new(576.0) + t4966 * t488 / F::new(3072.0) - t1174 * t4969 / F::new(144.0) - t1227 * t4974 / F::new(2304.0) + t3506 * t4980 / F::new(1536.0) - t3515 * t4984 / F::new(3072.0) + F::new(5.0) / F::new(13824.0) * t1227 * t4989 - t4994 / F::new(6912.0) + t4998 / F::new(4608.0) + t5002 * t1218 / F::new(3072.0) - t5005 * t1232 / F::new(4608.0) + t3536 * t1737 / F::new(3072.0);
    (t4997, t5000, t5002, t5005, t5010)
}
