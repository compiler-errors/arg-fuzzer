
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13306(_: S7, _: S1, _: S1) {}
        
        fn test13306() { foo13306(S2, S4, S1, S7, S2, S5, S7); }
    