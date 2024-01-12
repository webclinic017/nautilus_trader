# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2024 Nautech Systems Pty Ltd. All rights reserved.
#  https://nautechsystems.io
#
#  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
#  You may not use this file except in compliance with the License.
#  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
# -------------------------------------------------------------------------------------------------

from typing import Annotated

from msgspec import Meta


# An integer constrained to values > 0
PositiveInt = Annotated[int, Meta(gt=0)]

# An integer constrained to values >= 0
NonNegativeInt = Annotated[int, Meta(ge=0)]

# A float constrained to values > 0
PositiveFloat = Annotated[float, Meta(gt=0.0)]

# A float constrained to values >= 0
NonNegativeFloat = Annotated[float, Meta(ge=0.0)]