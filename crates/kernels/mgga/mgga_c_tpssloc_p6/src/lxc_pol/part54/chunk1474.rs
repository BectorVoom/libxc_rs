//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1474/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1474<F: Float>(t122609: F, t122610: F, t122623: F, t122625: F, t122627: F, t122645: F, t122656: F, t122659: F, t122662: F, t122664: F, t26880: F, t27180: F, t27888: F, t33746: F, t7218: F, t7266: F, t7806: F, t8690: F) -> F {
    let t125003 = -t26880 * t8690 - F::cast_from(2.0_f64) * t27180 * t7266 - F::cast_from(2.0_f64) * t27888 * t7806 + t33746 * t7218 - F::cast_from(2.0_f64) * t122609 - F::cast_from(2.0_f64) * t122610 - t122623 - t122625 - t122627 - t122645 + t122656 - t122659 - t122662 - t122664;
    t125003
}
