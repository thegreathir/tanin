"""Custom topology example

Two directly connected switches plus a host for each switch:

   host --- switch --- switch --- host

Adding the 'topos' dict with a key/value pair to generate our newly defined
topology enables one to pass in '--topo=mytopo' from the command line.
"""

from mininet.topo import Topo

class MyTopo( Topo ):
    "Simple topology example."

    def build( self ):
        "Create custom topo."

        # Add hosts and switches
        leftHost = self.addHost( 'hl' )
        middleHost = self.addHost( 'hm' )
        rightHost = self.addHost( 'hr' )

        # Add links
        self.addLink( leftHost, middleHost )
        self.addLink( middleHost, rightHost )

topos = { 'mytopo': ( lambda: MyTopo() ) }
