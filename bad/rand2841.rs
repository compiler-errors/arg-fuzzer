
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2841(_: S4, _: S2) {}
        
        fn test2841() { foo2841(S3, S4, S5); }
    