
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10179(_: S2, _: S4, _: S5, _: S7) {}
        
        fn test10179() { foo10179(S1, S4, S6, S2, S3, S2); }
    