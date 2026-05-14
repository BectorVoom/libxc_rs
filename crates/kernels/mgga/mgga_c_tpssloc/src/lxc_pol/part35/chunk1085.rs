//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1085/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1085<F: Float>(t28067: F, t6936: F, t22839: F, t6371: F, t1998: F, t236: F, t6330: F, t22845: F, t6347: F, t6926: F, t6375: F, t6916: F, t26246: F, t26268: F, t27012: F, t27019: F, t27022: F, t27027: F, t28058: F, t28061: F, t28063: F, t28065: F) -> (F, F, F) {
    let t28068 = t6936 * t28067;
    let t28070 = t22839 * t6371;
    let t28073 = t1998 * t236 * t6330;
    let t28074 = t22845 * t28073;
    let t28077 = t1998 * t236 * t6347;
    let t28078 = t6926 * t28077;
    let t28080 = t6916 * t6375;
    let t28083 = t27012 + 0.6728792682356731809e-4 * t26246 - t27019 + 0.40372756094140390854e-3 * t28058 - 0.20186378047070195427e-3 * t28061 - t28063 / 1536.0 - t28065 / 768.0 - 0.20186378047070195427e-3 * t28068 + t27022 + t28070 / 16.0 + 0.84782787797694820792e-2 * t28074 - 0.12111826828242117256e-2 * t28078 - t28080 / 48.0 + t27027 + 0.16956557559538964159e-1 * t26268;
    (t28073, t28077, t28083)
}
