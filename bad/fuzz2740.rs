
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2740(_: S1, _: S6, _: S4, _: S3, _: S7, _: S8) {}
        
        fn test2740() { foo2740(S7, S1, S7, S1, S1, S6, S4, S1); }
    