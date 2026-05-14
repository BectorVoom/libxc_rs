//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1230/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1230<F: Float>(t31918: F, t7458: F, t2314: F, t33735: F, t4034: F, t1873: F, t27858: F, t652: F, t33746: F, t6997: F, t120092: F, t120095: F, t120097: F, t120103: F, t120104: F, t120107: F, t123178: F, t5361: F, t8687: F) -> (F,) {
    let t123180 = t7458 * t31918;
    let t123182 = t2314 * t33735;
    let t123184 = t4034 * t33735;
    let t123187 = t652 * t27858 * t1873;
    let t123189 = t33746 * t6997;
    let t123191 = t5361 * t8687 - t120092 + t120095 - t120097 + t120103 - 3.0 * t120104 + t120107 - 3.0 * t123178 - 2.0 * t123180 - 2.0 * t123182 - 2.0 * t123184 - 2.0 * t123187 + t123189;
    (t123191,)
}
