
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15281(_: S2, _: S5, _: S7) {}
        
        fn test15281() { foo15281(S1, S2, S7, S8); }
    