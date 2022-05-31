
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15856(_: S6, _: S2, _: S2, _: S4, _: S0) {}
        
        fn test15856() { foo15856(S1, S6, S2, S1, S1, S7); }
    