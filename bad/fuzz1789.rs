
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1789(_: S1, _: S2, _: S4, _: S6, _: S7) {}
        
        fn test1789() { foo1789(S4, S6, S5, S1, S2, S4, S8, S1); }
    