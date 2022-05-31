
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1204(_: S8, _: S6, _: S4) {}
        
        fn test1204() { foo1204(S5, S8, S6, S5, S8, S5, S1); }
    