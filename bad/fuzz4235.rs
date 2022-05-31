
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4235(_: S6, _: S7) {}
        
        fn test4235() { foo4235(S1, S2, S3, S4, S6, S8); }
    