//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 940/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk940<F: Float>(t76741: F, t7720: F, t73851: F, t73854: F, t73865: F, t73871: F, t73873: F, t73875: F, t73877: F, t73879: F, t73857: F, t73862: F, t76713: F, t76718: F, t76723: F, t76728: F, t76733: F, t76738: F) -> F {
    let t76742 = t7720 * t76741;
    let t76743 = F::new(0.42564599893297839398e-5) * t76742;
    let t76744 = F::new(0.2627895913935205078e-5) * t73851;
    let t76745 = F::new(0.2627895913935205078e-5) * t73854;
    let t76748 = F::new(0.19709219354514038085e-5) * t73865;
    let t76749 = F::new(0.64054962902170623776e-5) * t73871;
    let t76750 = F::new(0.85129199786595678799e-5) * t73873;
    let t76751 = F::new(0.2553875993597870364e-4) * t73875;
    let t76752 = F::new(0.2553875993597870364e-4) * t73877;
    let t76753 = F::new(0.1702583995731913576e-4) * t73879;
    let t76754 = -t76713 + t76718 - t76723 + t76728 - t76733 - t76738 + t76743 + t76744 - t76745 - F::new(0.87596530464506835935e-6) * t73857 + F::new(0.87596530464506835935e-6) * t73862 - t76748 - t76749 + t76750 - t76751 + t76752 + t76753;
    t76754
}
