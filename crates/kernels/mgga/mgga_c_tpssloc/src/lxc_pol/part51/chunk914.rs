//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 914/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk914<F: Float>(t23042: F, t23063: F, t23070: F, t23084: F, t25065: F, t25069: F, t25071: F, t25073: F, t25077: F, t25080: F, t25103: F, t25107: F, t25109: F, t25113: F, t25117: F, t25121: F, t25124: F, t25126: F, t25128: F, t25133: F, t25136: F, t25158: F) -> (F,) {
    let t25160 = 0.20186378047070195427e-3 * t25065 + 7.0 / 2304.0 * t23042 - t25069 / 384.0 - t25071 / 384.0 - t25073 / 384.0 + 0.84782787797694820794e-2 * t23063 + 7.0 / 144.0 * t23070 + 7.0 / 576.0 * t25077 + 0.14130464632949136799e-2 * t23084 - 7.0 / 2304.0 * t25080 + t25103 - 0.12111826828242117256e-2 * t25107 + 0.84782787797694820792e-2 * t25109 + 0.12111826828242117256e-2 * t25113 - 0.20186378047070195427e-3 * t25117 + 0.84782787797694820792e-2 * t25121 - 0.20186378047070195427e-3 * t25124 + 0.14130464632949136799e-2 * t25126 - t25128 / 48.0 + 0.33643963411783659045e-4 * t25133 + t25136 / 1536.0 + t25158;
    (t25160,)
}
