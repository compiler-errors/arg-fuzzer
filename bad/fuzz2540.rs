
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2540(_: S1, _: S6, _: S7) {}
        
        fn test2540() { foo2540(S4, S5, S6, S5); }
    