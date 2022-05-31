
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11343(_: S4, _: S5, _: S6, _: S8) {}
        
        fn test11343() { foo11343(S4, S3, S4, S0, S5, S1); }
    