//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1067/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1067<F: Float>(t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t21781: F, t21783: F, t21786: F, t21789: F, t21792: F, t21795: F, t21802: F, t21804: F) -> F {
    let t21937 = F::new(0.16504875e0) * t21781 + F::new(0.258925e1) * t21783 + F::new(0.19419375e1) * t21786 - F::new(0.16557e0) * t21789 + F::new(0.49671e0) * t21792 + F::new(0.82785e-1) * t21795 + F::cast_from(0.33547222222222222222e0_f64) * t21760 - F::new(0.12077e1) * t21764 + F::new(0.181155e1) * t21771 + F::new(0.301925e0) * t21778 + F::cast_from(0.36793333333333333333e-1_f64) * t21802 - F::cast_from(0.412621875e-1_f64) * t21804 - F::cast_from(0.60384999999999999999e0_f64) * t21767 + F::new(0.181155e1) * t21774;
    t21937
}
