//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1307/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1307<F: Float>(t225: F, t814: F, t6648: F, t81612: F, t23021: F, t6547: F, t23151: F, t2613: F, t30684: F, t4281: F, t6660: F, t808: F, t81563: F, t81568: F, t81571: F, t81575: F, t81585: F, t81589: F, t81592: F, t81595: F, t81599: F, t81600: F, t81602: F, t81606: F, t81610: F, t9632: F) -> (F, F) {
    let t81613 = t225 * t814;
    let t81615 = t81612 * t81613 * t6648;
    let t81617 = t6547 * t23021;
    let t81621 = -F::new(0.9869604401089358619e-1) * t81563 + F::new(0.49348022005446793095e-1) * t81568 - F::new(0.12337005501361698274e-1) * t81571 + F::new(0.49348022005446793095e-1) * t81575 + F::new(3.0) * t808 * t23151 + F::new(6.0) * t4281 * t30684 * t9632 - F::new(0.14804406601634037928e0) * t81585 + F::new(0.49348022005446793095e-1) * t81589 - F::new(0.23029076935875170111e0) * t81592 - F::new(0.24674011002723396547e-1) * t81595 - t81599 + F::new(0.78134368175290755733e-1) * t81600 + F::new(0.19190897446562641759e0) * t81602 + F::new(0.9869604401089358619e-1) * t81606 + F::new(0.49348022005446793095e-1) * t81610 + F::new(0.24674011002723396547e-1) * t81615 - F::new(0.57572692339687925277e-1) * t81617 + F::new(3.0) * t2613 * t6660;
    (t81613, t81621)
}
