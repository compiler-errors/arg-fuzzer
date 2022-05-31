
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13077(_: S4, _: S5, _: S6) {}
        
        fn test13077() { foo13077(S3, S5, S4, S0); }
    