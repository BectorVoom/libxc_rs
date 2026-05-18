//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1116/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1116<F: Float>(t5416: F, t972: F, t135: F, t6187: F, t1174: F, t4889: F, t5040: F, t6183: F, t6177: F, t1198: F, t15484: F, t15488: F, t15490: F, t15494: F, t15498: F, t15524: F, t15550: F, t15574: F, t15580: F, t15737: F, t1748: F, t4980: F, t5024: F, t5030: F) -> (F, F) {
    let t18321 = t5416 * t972;
    let t18324 = t135 * t6187;
    let t18325 = t1174 * t18324;
    let t18327 = t4889 * t5040;
    let t18329 = t135 * t6183;
    let t18330 = t1174 * t18329;
    let t18332 = t135 * t6177;
    let t18333 = t1174 * t18332;
    let t18337 = t15498 * t1748 / F::new(432.0) + t5024 * t5030 / F::new(432.0) - t15484 - t15488 + t15490 + t15494 + t15524 - F::new(11.0) / F::new(324.0) * t18321 * t1198 - t18325 / F::new(432.0) + t18327 / F::new(162.0) - t18330 / F::new(864.0) - t15550 - t15574 + t18333 / F::new(648.0) - t15580 + t15737 * t4980 / F::new(768.0);
    (t18321, t18337)
}
